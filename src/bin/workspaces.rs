use winit::event_loop::EventLoopBuilder;

#[cfg(feature = "jap")]
const WS_NAMES: [&str; 10] = ["一", "ニ", "三", "四", "五", "六", "七", "八", "九", "十"];
#[cfg(not(feature = "jap"))]
const WS_NAMES: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
const SUFF: [&str; 10] = ["₀", "₁", "₂", "₃", "₄", "₅", "₆", "₇", "₈", "₉"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoopBuilder::new().build()?;
    let prime = event_loop.primary_monitor();
    for (idx, mon) in event_loop.available_monitors().enumerate() {
        let mut bspc = std::process::Command::new("bspc");
        if let Some(name) = &mon.name() {
            let name = if name.contains(".") {
                format!("%{name}").to_string()
            } else {
                name.to_string()
            };
            let mut args = vec!["monitor".to_string(), name, "-d".to_string()];
            if let Some(prime) = &prime {
                let iter = WS_NAMES.into_iter();
                if mon != *prime {
                    args.extend(iter.map(|ws| format!("{}{}", ws, SUFF[idx])));
                } else {
                    args.extend(iter.map(|ws| ws.to_string()));
                }
            }
            bspc.args(args)
                .status()
                .expect("could not setup workspaces for {mon:#?}");
        }
    }

    Ok(())
}
