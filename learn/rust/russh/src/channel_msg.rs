/// [russh::ChannelMsg]
pub enum ChannelMsg {
    /// [russh::ChannelMsg::Open]
    Open {
        /// [russh::ChannelMsg::Open::id]
        id: russh::ChannelId,
        /// [russh::ChannelMsg::Open::max_packet_size]
        max_packet_size: u32,
        /// [russh::ChannelMsg::Open::window_size]
        window_size: u32,
    },
    /// [russh::ChannelMsg::OpenFailure]
    OpenFailure(russh::ChannelOpenFailure),
    /// [russh::ChannelMsg::SetEnv]
    SetEnv {
        /// [russh::ChannelMsg::SetEnv::want_reply]
        want_reply: bool,
        /// [russh::ChannelMsg::SetEnv::variable_name]
        variable_name: String,
        /// [russh::ChannelMsg::SetEnv::variable_value]
        variable_value: String,
    },
    /// [russh::ChannelMsg::RequestPty]
    RequestPty {
        /// [russh::ChannelMsg::RequestPty::want_reply]
        want_reply: bool,
        /// [russh::ChannelMsg::RequestPty::term]
        term: String,
        /// [russh::ChannelMsg::RequestPty::col_width]
        col_width: u32,
        /// [russh::ChannelMsg::RequestPty::row_height]
        row_height: u32,
        /// [russh::ChannelMsg::RequestPty::pix_width]
        pix_width: u32,
        /// [russh::ChannelMsg::RequestPty::pix_height]
        pix_height: u32,
        /// [russh::ChannelMsg::RequestPty::terminal_modes]
        terminal_modes: Vec<(russh::Pty, u32)>,
    },
    /// [russh::ChannelMsg::Success]
    Success,
    /// [russh::ChannelMsg::Failure]
    Failure,
    /// [russh::ChannelMsg::RequestShell]
    RequestShell {
        /// [russh::ChannelMsg::RequestShell::want_reply]
        want_reply: bool,
    },
    /// [russh::ChannelMsg::Exec]
    Exec {
        /// [russh::ChannelMsg::Exec::want_reply]
        want_reply: bool,
        /// [russh::ChannelMsg::Exec::command]
        command: Vec<u8>,
    },
    /// [russh::ChannelMsg::Data]
    Data {
        /// [russh::ChannelMsg::Data::data]
        data: russh::CryptoVec,
    },
    /// [russh::ChannelMsg::ExitStatus]
    ExitStatus {
        /// [russh::ChannelMsg::ExitStatus::exit_status]
        exit_status: u32,
    },
    /// [russh::ChannelMsg::Close]
    Close,
}
