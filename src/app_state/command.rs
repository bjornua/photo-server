/*
How should the log work?

running the command for the first time:
    * run command with sideeffects
        * io, network, callback

replaying the log:
    * run command without sideeffects

command() {
    result(appState) -> Result;

    sideeffects(appstate) -> Result<(), Error>;

}

trait command:
    execute(state, params) -> result;
    updateState(state);
    sideffects(newState, result);
*/
