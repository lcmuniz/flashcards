use crate::ui::cli::executar_cli;

mod domain;
mod storage;
mod ui;

fn main() {
    if let Err(e) = executar_cli() {
        eprintln!("Erro: {:?}", e);
    }
}
