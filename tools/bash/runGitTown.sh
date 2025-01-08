#! /bin/bash

ACTION_NAME="${1}"

if [ -z "${ACTION_NAME}" ]; then
    echo "An action is required. Valid actions are: 'switch', 'sync', 'sync-all', 'hack', 'hack-prototype'"
    exit 1
fi

if ! [[ "${ACTION_NAME}" =~ ^(switch|sync|sync-all|hack|hack-prototype)$ ]]; then
    echo "Invalid action. Valid actions are: 'switch', 'sync', 'sync-all', 'hack', 'hack-prototype'"
    exit 1
fi

case "${ACTION_NAME}" in
    "switch")
        git town switch
        ;;
    "sync")
        git town sync
        ;;
    "sync-all")
        git town sync --all
        ;;
    "hack")
        read -p "Enter the name of the new branch: " BRANCH_NAME
        git town hack "${BRANCH_NAME}"
        ;;
    "hack-prototype")
        read -p "Enter the name of the new branch: " BRANCH_NAME
        git town hack --prototype "${BRANCH_NAME}"
        ;;
esac
