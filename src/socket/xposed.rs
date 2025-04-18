use anyhow::Result;

pub trait Xposed {
    fn GetXposedStatus(&self) -> Result<bool>;
    fn GetXposedLog(&self) -> Result<String>;
}

impl Xposed for super::SocketLog {
    fn GetXposedStatus(&self) -> Result<bool> {
        let len = self.ReceiveLog()?;
        if len.is_empty() {
            self.SendLog("未能获取Xposed日志，请确认LSPosed中Frozen是否已经勾选系统框架")?;
            log::error!("未能获取Xposed日志，请确认LSPosed中Frozen是否已经勾选系统框架");
            return Ok(false);
        }
        Ok(true)
    }

    fn GetXposedLog(&self) -> Result<String> {
        let len = self.ReceiveLog()?;
        return Ok(len);
    }
}
