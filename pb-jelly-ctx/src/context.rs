//! Context is an immutable map of key-value pairs inspired by Go. It can be used as a shortcut
//! to passing around request-scope "constants" such as `request_id`, client identity, etc.
//!
//! Example of usage:
//! ```ignore
//! lazy_static! {
//!     static ref REQUEST_ID_KEY: ContextKey<String> = ContextKey::new("request_id");
//! }
//!
//! fn with_request_id(ctx: Context, request_id: String) -> Context {
//!     ctx.with_value(&REQUEST_ID_KEY, request_id)
//! }
//!
//! fn request_id(ctx: &Context) -> Option<&String> {
//!     ctx.value(*REQUEST_ID_KEY)
//! }
//! ```

use std::any::Any;
use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use std::sync::Arc;
use std::time::{
    Duration,
    SystemTime,
};

lazy_static! {
    static ref TODO: Context = Context::new_empty("Context::TODO()");
    static ref BACKGROUND: Context = Context::new_empty("Context::background()");
    static ref KEY_COUNTER: AtomicUsize = AtomicUsize::new(1);
}

#[derive(Clone, Copy)]
struct ContextId {
    id: usize,
    name: &'static str,
}

impl PartialEq for ContextId {
    fn eq(&self, other: &ContextId) -> bool {
        self.id == other.id
    }
}

impl fmt::Debug for ContextId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {}]", self.id, self.name)
    }
}

impl ContextId {
    pub fn new(name: &'static str) -> Self {
        let id = KEY_COUNTER.fetch_add(1, Ordering::Relaxed);
        ContextId { id, name }
    }
}

pub trait ContextValue: fmt::Debug + Send + Sync + 'static {}

impl<T> ContextValue for T where T: fmt::Debug + Send + Sync + 'static {}

pub struct ContextKey<T>
where
    T: ContextValue,
{
    key: ContextId,
    _phantom: PhantomData<T>,
}

impl<T> ContextKey<T>
where
    T: ContextValue,
{
    pub fn new(name: &'static str) -> Self {
        let key = ContextId::new(name);
        ContextKey {
            key,
            _phantom: Default::default(),
        }
    }
}

impl<T> fmt::Debug for ContextKey<T>
where
    T: ContextValue,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key.fmt(f)
    }
}

struct ContextItem<T>
where
    T: ContextValue,
{
    value: T,
}

impl<T> fmt::Debug for ContextItem<T>
where
    T: ContextValue,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

trait ContextValueWrapper: ContextValue {
    fn as_any(&self) -> &dyn Any;
}

impl<T> ContextValueWrapper for ContextItem<T>
where
    T: ContextValue,
{
    fn as_any(&self) -> &dyn Any {
        &self.value
    }
}

enum Inner {
    Root {
        name: &'static str,
    },
    Deadline {
        parent: Arc<Inner>,
        deadline: SystemTime,
    },
    Value {
        parent: Arc<Inner>,
        // Deadline is present here as optimization.
        deadline: Option<SystemTime>,
        key: ContextId,
        value: Box<dyn ContextValueWrapper>,
    },
}

impl Inner {
    fn value<T>(&self, target: &ContextKey<T>) -> Option<&T>
    where
        T: ContextValue,
    {
        match *self {
            Inner::Root { .. } => None,
            Inner::Deadline { ref parent, .. } => parent.value(target),
            Inner::Value {
                ref parent,
                key,
                ref value,
                ..
            } => {
                if key == target.key {
                    Some(value.as_any().downcast_ref::<T>().unwrap())
                } else {
                    parent.value(target)
                }
            },
        }
    }

    fn deadline(&self) -> Option<SystemTime> {
        match *self {
            Inner::Root { .. } => None,
            Inner::Deadline { ref deadline, .. } => Some(*deadline),
            Inner::Value { ref deadline, .. } => *deadline,
        }
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Inner::Root { name } => write!(f, "{}", name),
            Inner::Deadline {
                ref parent,
                ref deadline,
            } => {
                let timeout = deadline
                    .duration_since(SystemTime::now())
                    .unwrap_or_else(|_| Duration::new(0, 0));
                parent.fmt(f)?;
                write!(f, ".with_deadline({:?} [{:?}])", deadline, timeout)
            },
            Inner::Value {
                ref parent,
                key,
                ref value,
                ..
            } => {
                parent.fmt(f)?;
                write!(f, ".with_value({:?}, {:?})", key, value)
            },
        }
    }
}

#[derive(Clone)]
pub struct Context {
    inner: Arc<Inner>,
}

impl Context {
    fn new_empty(name: &'static str) -> Self {
        Context {
            inner: Arc::new(Inner::Root { name }),
        }
    }

    pub fn background() -> Self {
        BACKGROUND.clone()
    }

    #[allow(non_snake_case)]
    pub fn TODO() -> Self {
        TODO.clone()
    }

    pub fn with_timeout(&self, timeout: Duration) -> Self {
        self.with_deadline(SystemTime::now() + timeout)
    }

    pub fn with_deadline(&self, deadline: SystemTime) -> Self {
        if let Some(self_deadline) = self.inner.deadline() {
            if self_deadline < deadline {
                // The current deadline is already sooner.
                return self.clone();
            }
        }

        Context {
            inner: Arc::new(Inner::Deadline {
                parent: Arc::clone(&self.inner),
                deadline,
            }),
        }
    }

    pub fn with_value<T>(&self, key: &ContextKey<T>, value: T) -> Self
    where
        T: ContextValue,
    {
        Context {
            inner: Arc::new(Inner::Value {
                parent: Arc::clone(&self.inner),
                deadline: self.inner.deadline(),
                key: key.key,
                value: Box::new(ContextItem { value }),
            }),
        }
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.deadline().map(|deadline| {
            let now = SystemTime::now();
            if now < deadline {
                deadline.duration_since(now).unwrap()
            } else {
                Duration::new(0, 0)
            }
        })
    }

    pub fn deadline(&self) -> Option<SystemTime> {
        self.inner.deadline()
    }

    pub fn done(&self) -> bool {
        // TODO(elessar): Allow to pass knowledge about cancellation?..
        if let Some(deadline) = self.deadline() {
            deadline <= SystemTime::now()
        } else {
            false
        }
    }

    pub fn value<T>(&self, key: &ContextKey<T>) -> Option<&T>
    where
        T: ContextValue,
    {
        self.inner.value(key)
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref FIRST_TEST_KEY: ContextKey<String> = ContextKey::new("test1");
        static ref SECOND_TEST_KEY: ContextKey<String> = ContextKey::new("test2");
    }

    #[test]
    fn test_background() {
        let ctx = Context::background();
        assert_eq!(ctx.timeout(), None);
        assert_eq!(ctx.deadline(), None);
        assert!(ctx.value(&FIRST_TEST_KEY).is_none());
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());
    }

    #[test]
    fn test_todo() {
        let ctx = Context::TODO();
        assert_eq!(ctx.timeout(), None);
        assert_eq!(ctx.deadline(), None);
        assert!(ctx.value(&FIRST_TEST_KEY).is_none());
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());
    }

    #[test]
    fn test_with_expired_deadline() {
        let now = SystemTime::now();
        let deadline = now - Duration::from_secs(5);

        let ctx = Context::background().with_deadline(deadline);
        assert_eq!(ctx.deadline().unwrap(), deadline);
        assert_eq!(ctx.timeout().unwrap(), Duration::from_secs(0));
        assert!(ctx.done());
    }

    #[test]
    fn test_with_deadline() {
        let now = SystemTime::now();

        let ctx = Context::background().with_deadline(now + Duration::from_secs(5));
        assert_eq!(ctx.deadline().unwrap(), now + Duration::from_secs(5));

        let timeout = ctx.timeout().unwrap();
        assert!(timeout < Duration::from_secs(6));
        assert!(timeout > Duration::from_secs(4));
        assert!(!ctx.done());

        assert!(ctx.value(&FIRST_TEST_KEY).is_none());
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());
    }

    #[test]
    fn test_with_value() {
        let ctx = Context::background().with_value(&FIRST_TEST_KEY, "aaa".into());

        assert_eq!(ctx.value(&FIRST_TEST_KEY).unwrap(), "aaa");
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());
        assert!(ctx.deadline().is_none());

        let deadline = SystemTime::now() + Duration::from_secs(5);
        let ctx = ctx.with_deadline(deadline);

        let ctx2 = ctx.with_value(&FIRST_TEST_KEY, "bbb".into());
        assert_eq!(ctx.value(&FIRST_TEST_KEY).unwrap(), "aaa");
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());

        assert_eq!(ctx2.value(&FIRST_TEST_KEY).unwrap(), "bbb");
        assert!(ctx2.value(&SECOND_TEST_KEY).is_none());

        let ctx3 = ctx.with_value(&SECOND_TEST_KEY, "ccc".into());
        assert_eq!(ctx.value(&FIRST_TEST_KEY).unwrap(), "aaa");
        assert!(ctx.value(&SECOND_TEST_KEY).is_none());

        assert_eq!(ctx2.value(&FIRST_TEST_KEY).unwrap(), "bbb");
        assert!(ctx2.value(&SECOND_TEST_KEY).is_none());

        assert_eq!(ctx3.value(&FIRST_TEST_KEY).unwrap(), "aaa");
        assert_eq!(ctx3.value(&SECOND_TEST_KEY).unwrap(), "ccc");

        assert_eq!(ctx2.deadline().unwrap(), deadline);
    }
}
