# auto complete script for kanben
# https://github.com/benbrunton/kanben

_kanben_completion() {
    COMMANDS="add clear-done complete delete edit help now reindex start tag tasks top view"
    COMMANDS_WITH_TASK_PARAM=(
        "complete"
        "delete" 
        "edit"
        "start"
        "tag"
        "top"
        "view"
    )

    WORD_COUNT=${#COMP_WORDS[@]}
    COMMAND_IN_LIST=false

    

    # root commands
    if [ $WORD_COUNT -le 2 ]; then
        COMPREPLY=($(compgen -W "$COMMANDS" "${COMP_WORDS[1]}"))
    elif [ $WORD_COUNT -le 3 ]; then
        for i in "${COMMANDS_WITH_TASK_PARAM[@]}"
        do
            if [ "$i" == "${COMP_WORDS[1]}" ]; then
                COMPREPLY=($(compgen -W "$(kanben tasks)" "${COMP_WORDS[2]}"))
            fi
        done
    fi
}

complete -F _kanben_completion kanben
