mod driver;
mod rgb;

use anyhow::Result;
use clap::{Parser, Subcommand};
use driver::Magi96Driver;
use rgb::{HsvColor, RgbEffect};

#[derive(Parser)]
#[command(name = "magi96-keyboard")]
#[command(about = "Driver for IQUNIX Magi96 keyboard RGB control", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set RGB backlight brightness (0-9)
    Brightness {
        /// Brightness level (0-9)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=9))]
        level: u8,
    },
    
    /// Set RGB effect
    Effect {
        /// Effect name (e.g., wave, breathe, vortex)
        effect: String,
    },
    
    /// Set RGB effect speed (0-4)
    Speed {
        /// Speed level (0-4, where 0 is slowest)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=4))]
        speed: u8,
    },
    
    /// Set RGB color using HSV values
    Color {
        /// Hue (0-255)
        hue: u8,
        /// Saturation (0-255)
        saturation: u8,
        /// Value/Brightness (0-255)
        value: u8,
    },
    
    /// List all available RGB effects
    ListEffects,
    
    /// Show keyboard device information
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::ListEffects => {
            println!("Available RGB effects:");
            for (i, effect) in RgbEffect::all().iter().enumerate() {
                println!("  {:2}. {} ({})", i, effect, format!("{:?}", effect).to_lowercase());
            }
            Ok(())
        }
        
        _ => {
            let driver = Magi96Driver::open()?;
            
            match cli.command {
                Commands::Brightness { level } => {
                    driver.set_brightness(level)?;
                    println!("✓ Brightness set to {}", level);
                }
                
                Commands::Effect { effect } => {
                    let rgb_effect = RgbEffect::from_name(&effect)
                        .ok_or_else(|| anyhow::anyhow!(
                            "Unknown effect '{}'. Use 'list-effects' to see available effects.",
                            effect
                        ))?;
                    
                    driver.set_effect(rgb_effect)?;
                    println!("✓ Effect set to {}", rgb_effect);
                }
                
                Commands::Speed { speed } => {
                    driver.set_effect_speed(speed)?;
                    println!("✓ Effect speed set to {}", speed);
                }
                
                Commands::Color { hue, saturation, value } => {
                    let color = HsvColor::new(hue, saturation, value);
                    driver.set_color(color)?;
                    println!("✓ Color set to HSV({}, {}, {})", hue, saturation, value);
                }
                
                Commands::Info => {
                    let info = driver.get_device_info()?;
                    println!("Magi96 Keyboard Information:");
                    println!("{}", info);
                }
                
                Commands::ListEffects => unreachable!(),
            }
            
            Ok(())
        }
    }
}
