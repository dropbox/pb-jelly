#!/bin/sh

# Welcome Messages
printf "Thanks for your interest in developing on \e[34mpb-rs\e[0m!\n\n"
printf "This script will run the following commands to install the necessary packages, and generate necessary code, for development\n\n"

# Describing the packages we install
printf "\t -> \e[4mbrew\e[24m install \e[35mprotobuf go coreutils\e[0m\n"
printf "\t -> \e[4mpip\e[24m install \e[35msix protobuf\e[0m\n"
printf "\t -> \e[4mgo\e[24m get github.com/\e[35mgogo/protobuf/proto\e[0m\n"
printf "\n\t -> ./\e[35mgen_protos.sh\e[0m\n\n"

# Ask for consent
while true; do
    read -p "Continue, and install the above packages? [y/N] " yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) exit;;
        * ) echo "Please answer [Y/y] or [N/n].";;
    esac
done

# Show output of the commands we run
set -ex

# Run the install commands
brew install protobuf go coreutils
pip install six protobuf
go get github.com/gogo/protobuf/proto
./gen_protos.sh
