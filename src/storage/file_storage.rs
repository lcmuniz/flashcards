use crate::domain;
use std::str;

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::Io(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::Serde(err)
    }
}

#[derive(Debug, Clone)]
pub struct FileStorage {
    pub caminho_arquivo: String,
}

impl FileStorage {
    pub fn novo(caminho_arquivo: &str) -> Self {
        Self {
            caminho_arquivo: caminho_arquivo.to_string(),
        }
    }

    pub fn listar(&self) -> Result<Vec<domain::Flashcard>, StorageError> {
        let conteudo = std::fs::read_to_string(&self.caminho_arquivo)?;
        let flashcards: Vec<domain::Flashcard> = serde_json::from_str(&conteudo)?;
        Ok(flashcards)
    }

    pub fn obter(&self, id: uuid::Uuid) -> Result<Option<domain::Flashcard>, StorageError> {
        let flashcards = self.listar()?;
        Ok(flashcards.into_iter().find(|f| f.id == id))
    }

    pub fn adicionar(&self, flashcard: &domain::Flashcard) -> Result<(), StorageError> {
        let mut flashcards = self.listar().unwrap_or_else(|_| Vec::new());
        // verifica se ja existe. se sim, retorna erro
        if let Some(pos) = flashcards.iter().position(|f| f.id == flashcard.id) {
            return Err(StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Flashcard com esse ID já existe.",
            )));
        } else {
            flashcards.push(flashcard.clone());
        }
        let conteudo = serde_json::to_string_pretty(&flashcards)?;
        std::fs::write(&self.caminho_arquivo, conteudo)?;
        Ok(())
    }

    pub fn atualizar(&self, flashcard: &domain::Flashcard) -> Result<(), StorageError> {
        let mut flashcards = self.listar()?;
        if let Some(pos) = flashcards.iter().position(|f| f.id == flashcard.id) {
            flashcards[pos] = flashcard.clone();
        } else {
            return Err(StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Flashcard não encontrado para atualização.",
            )));
        }
        let conteudo = serde_json::to_string_pretty(&flashcards)?;
        std::fs::write(&self.caminho_arquivo, conteudo)?;
        Ok(())
    }

    pub fn remover(&self, id: uuid::Uuid) -> Result<(), StorageError> {
        let mut flashcards = self.listar()?;
        if let Some(pos) = flashcards.iter().position(|f| f.id == id) {
            flashcards.remove(pos);
        } else {
            return Err(StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Flashcard não encontrado para remoção.",
            )));
        }
        let conteudo = serde_json::to_string_pretty(&flashcards)?;
        std::fs::write(&self.caminho_arquivo, conteudo)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Flashcard;
    use std::fs;

    #[test]
    fn test_adicionar_e_listar_flashcards() {
        let caminho_arquivo = "test_flashcards_1.json";
        let storage = FileStorage::novo(caminho_arquivo);

        // Limpa o arquivo de teste antes de começar
        let _ = fs::remove_file(caminho_arquivo);

        let flashcard = Flashcard::novo("Pergunta 1", "Resposta 1").unwrap();
        storage.adicionar(&flashcard).unwrap();

        let flashcards = storage.listar().unwrap();
        assert_eq!(flashcards.len(), 1);
        assert_eq!(flashcards[0].pergunta, "Pergunta 1");
        assert_eq!(flashcards[0].resposta, "Resposta 1");

        // Limpa o arquivo de teste após o teste
        let _ = fs::remove_file(caminho_arquivo);
    }

    #[test]
    fn test_atualizar_flashcard() {
        let caminho_arquivo = "test_flashcards_2.json";
        let storage = FileStorage::novo(caminho_arquivo);

        // Limpa o arquivo de teste antes de começar
        let _ = fs::remove_file(caminho_arquivo);

        let mut flashcard = Flashcard::novo("Pergunta 1", "Resposta 1").unwrap();
        storage.adicionar(&flashcard).unwrap();

        flashcard.atualizar_pergunta("Pergunta Atualizada").unwrap();
        storage.atualizar(&flashcard).unwrap();

        let flashcards = storage.listar().unwrap();
        assert_eq!(flashcards.len(), 1);
        assert_eq!(flashcards[0].pergunta, "Pergunta Atualizada");

        // Limpa o arquivo de teste após o teste
        let _ = fs::remove_file(caminho_arquivo);
    }

    #[test]
    fn test_remover_flashcard() {
        let caminho_arquivo = "test_flashcards_3.json";
        let storage = FileStorage::novo(caminho_arquivo);

        // Limpa o arquivo de teste antes de começar
        let _ = fs::remove_file(caminho_arquivo);

        let flashcard = Flashcard::novo("Pergunta 1", "Resposta 1").unwrap();
        storage.adicionar(&flashcard).unwrap();

        storage.remover(flashcard.id).unwrap();

        let flashcards = storage.listar().unwrap();
        assert_eq!(flashcards.len(), 0);

        // Limpa o arquivo de teste após o teste
        let _ = fs::remove_file(caminho_arquivo);
    }
}
