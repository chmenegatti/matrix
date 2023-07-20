use crossterm::{
  cursor,
  style::{Color, ResetColor, SetBackgroundColor},
  terminal::{self, ClearType},
  ExecutableCommand,
  Result,
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

const MATRIX_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn main() -> Result<()> {
  // Configuração inicial do terminal
  terminal::enable_raw_mode()?;
  stdout().execute(cursor::Hide)?;

  // Loop principal do protetor de tela
  loop {
      render_matrix_animation()?;
      sleep(Duration::from_millis(100));
  }
}

fn render_matrix_animation() -> Result<()> {
  // Obtém as dimensões do terminal
  let (width, height) = terminal::size()?;

  // Limpa o terminal
  stdout().execute(terminal::Clear(ClearType::All))?;

  for col in 0..width {
      for row in 0..height {
          // Define a cor de fundo como preto
          stdout().execute(SetBackgroundColor(Color::Black))?;

          // Exibe um caractere aleatório da "matriz" em verde em uma posição aleatória
          let matrix_char = MATRIX_CHARS.chars().nth(rand::random::<usize>() % MATRIX_CHARS.len()).unwrap();
          let color_code = format!("#{:02x}{:02x}{:02x}", 0, 255, 0);
          print_at(col, row, color_code, matrix_char)?;
      }
  }

  // Força a exibição imediata do buffer de saída
  stdout().flush()?;
  Ok(())
}

fn print_at(col: u16, row: u16, _color_code: String, ch: char) -> Result<()> {
  // Move o cursor para a posição especificada
  stdout().execute(cursor::MoveTo(col, row))?;

  // Define a cor do caractere
  print!(
      "{}{}{}",
      SetBackgroundColor(Color::Black),
      SetBackgroundColor(Color::Rgb { r: 0, g: 255, b: 0 }),
      ch
  );

  // Reseta as cores para evitar "vazamento" de cores para o próximo caractere
  stdout().execute(ResetColor)?;

  Ok(())
}

// Função auxiliar para garantir que a restauração do terminal seja executada
// mesmo em caso de erro ou interrupção do programa
// fn cleanup() {
//   stdout().execute(cursor::Show).unwrap();
//   terminal::disable_raw_mode().unwrap();
// }
