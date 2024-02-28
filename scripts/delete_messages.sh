#!/bin/bash

echo "Checking for MongoDB Shell (mongosh)..."

install_mongosh_debian() {
    sudo apt-get update -qq
    sudo apt-get install -y mongodb-mongosh
}

install_mongosh_redhat() {
    sudo yum -y install mongodb-mongosh
}

install_mongosh_mac() {
    brew install mongosh
}

if ! command -v mongosh &> /dev/null; then
    OS=$(uname -s)
    case "$OS" in
        Linux*)
            DISTRO=$(grep -Ei 'debian|buntu|mint' /etc/*release)
            if [[ ! -z "$DISTRO" ]]; then
                echo "Detected Debian-based Linux distribution."
                install_mongosh_debian
            else
                echo "Detected Red Hat-based Linux distribution."
                install_mongosh_redhat
            fi
            ;;
        Darwin*)
            echo "Detected macOS."
            install_mongosh_mac
            ;;
        *)
            echo "Unsupported operating system: $OS"
            exit 1
            ;;
    esac
else
    echo "mongosh is already installed."
fi

MONGODB_URI=$(grep "mongodb_uri" ./infra/terraform.tfvars | cut -d '"' -f2)
DATABASE_NAME=$(grep "bot_name" ./infra/terraform.tfvars | cut -d '"' -f2)
MONGO_COMMAND="db.getSiblingDB('$DATABASE_NAME').messages.deleteMany({})"

mongosh "$MONGODB_URI" --eval "$MONGO_COMMAND"

echo "The 'messages' collection in the '$DATABASE_NAME' database has been emptied."
