use super::Target;
use crate::{Error, HerdrWindow, Result};
use gpui::{Context, Window};
use herdr_client::{
    Method,
    protocol::{ClientShellCommandAction, ClientShellSnapshot},
};

fn bound_command<'a>(
    snapshot: &'a ClientShellSnapshot,
    binding: &str,
) -> Result<(&'a str, ClientShellCommandAction)> {
    snapshot
        .commands
        .iter()
        .find(|command| {
            command.binding_label == binding
                || command.binding_labels.iter().any(|label| label == binding)
        })
        .map(|command| (command.command_id.as_str(), command.action))
        .ok_or_else(|| Error::UnboundDaemonCommand(binding.to_owned()))
}

impl HerdrWindow {
    pub(crate) fn run_daemon_command(
        &mut self,
        action: &crate::actions::RunDaemonCommand,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.run_daemon_binding(&action.binding, window, cx);
    }

    fn run_daemon_binding(&mut self, binding: &str, _window: &mut Window, cx: &mut Context<Self>) {
        if self.menu.page.is_some() {
            return;
        }
        let Some(snapshot) = self.live.snapshot.clone() else {
            return;
        };
        let result = bound_command(&snapshot, binding)
            .and_then(|(id, action)| Target::capture(&snapshot).invocation(&snapshot, id, action));
        match result {
            Ok(params) => {
                self.request_focus_change(Method::CommandInvoke.as_str(), None, |handle, boot| {
                    handle.request(boot, Method::CommandInvoke, params)
                });
            }
            Err(error) => {
                self.local_error = Some(error.to_string());
                cx.notify();
            }
        }
    }
}

#[cfg(test)]
mod tests;
