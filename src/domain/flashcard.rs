use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Flashcard {
    pub id: Uuid,
    pub pergunta: String,
    pub resposta: String,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

impl Flashcard {
    pub fn novo(pergunta: &str, resposta: &str) -> Result<Self, &'static str> {
        let pergunta = pergunta.trim().to_string();
        let resposta = resposta.trim().to_string();

        if pergunta.is_empty() {
            return Err("A pergunta não pode ser vazia.");
        }
        if resposta.is_empty() {
            return Err("A resposta não pode ser vazia.");
        }

        let agora = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            pergunta,
            resposta,
            criado_em: agora,
            atualizado_em: agora,
        })
    }

    pub fn atualizar_pergunta(&mut self, nova_pergunta: &str) -> Result<(), &'static str> {
        let nova = nova_pergunta.trim().to_string();
        if nova.is_empty() {
            return Err("A nova pergunta não pode ser vazia.");
        }
        self.pergunta = nova;
        self.atualizado_em = Utc::now();
        Ok(())
    }

    pub fn atualizar_resposta(&mut self, nova_resposta: &str) -> Result<(), &'static str> {
        let nova = nova_resposta.trim().to_string();
        if nova.is_empty() {
            return Err("A nova resposta não pode ser vazia.");
        }
        self.resposta = nova;
        self.atualizado_em = Utc::now();
        Ok(())
    }

    pub fn para_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn de_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cria_flashcard_valido() {
        let fc = Flashcard::novo("Pergunta?", "Resposta!").unwrap();
        assert_eq!(fc.pergunta, "Pergunta?");
        assert_eq!(fc.resposta, "Resposta!");
    }

    #[test]
    fn valida_campos_vazios() {
        assert!(Flashcard::novo("", "ok").is_err());
        assert!(Flashcard::novo("ok", "  ").is_err());
    }

    #[test]
    fn atualiza_campos() {
        let mut fc = Flashcard::novo("A", "B").unwrap();
        fc.atualizar_pergunta("Nova?").unwrap();
        fc.atualizar_resposta("Resposta nova").unwrap();
        assert_eq!(fc.pergunta, "Nova?");
        assert_eq!(fc.resposta, "Resposta nova");
    }

    #[test]
    fn serializa_e_desserializa() {
        let fc = Flashcard::novo("Q", "A").unwrap();
        let j = fc.para_json().unwrap();
        let fc2 = Flashcard::de_json(&j).unwrap();
        assert_eq!(fc, fc2);
    }
}
