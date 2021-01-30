use super::AppState;

/*
How should the log work?

running the command for the first time:
    * run command with side effects
        * io, network, callback

replaying the log:
    * run command without side effects

command() {
    result(appState) -> Result;

    side effects(app_state) -> Result<(), Error>;

}

trait command:
    execute(state, params) -> result;
    update_state(state);
    side_effects(newState, result) -> Result<(), Error>;
*/

trait Command<R, SE> {
    fn run(&self, state: &mut AppState) -> Result<R, SE> {
        let result = self.side_effects(&result, state)?;
        let result = self.update_state(state);
        return Ok(result);
    }

    fn replay(&self, state: &mut AppState) -> Result<R, SE> {
        let result = self.update_state(state);
        return Ok(result);
    }

    fn update_state(&self, state: &mut AppState) -> R;
    fn side_effects(&self, result: &R, state: &AppState) -> Result<(), SE>;
}
