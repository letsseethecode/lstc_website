# Project dependencies are handled using `asdf` and the versions exist in the
# `.tool-versions` file in the root of th e project.
#
# TODO: This will only work on macs at the moment.  We need to get it working on Linux & WSL.

brew update || { echo "Brew update failed"; exit 1; }
brew install asdf || { echo "Brew failed to install asdf"; exit 1; }
brew install gh || { echo "Brew failed to install gh"; exit 1; }
brew install awscli || { echo "Brew failed to install awscli"; exit 1; }

# install rust
which rustup &> /dev/null
if [[ $? != 0 ]]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

# Install terraform
asdf plugin-add terraform || { echo "Failed to add terraform plugin"; exit 1; }
asdf install terraform || { echo "Failed to install terraform version"; exit 1; }
