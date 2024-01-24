# Project dependencies are handled using `asdf` and the versions exist in the
# `.tool-versions` file in the root of th e project.
#
# TODO: This will only work on macs at the moment.  We need to get it working on Linux & WSL.

brew update || { echo "Brew update failed"; exit 1; }
brew install asdf || { echo "Brew failed to install asdf"; exit 1; }

asdf plugin-add terraform || { echo "Failed to add terraform plugin"; exit 1; }

# Install terraform
asdf install terraform || { echo "Failed to install terraform version"; exit 1; }
