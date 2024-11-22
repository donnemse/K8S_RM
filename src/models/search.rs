#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchConfig {
    pub column: usize,
    pub word: [u8; 64], // 최대 64바이트 길이의 문자열을 저장
}

impl SearchConfig {
    pub fn new(column: usize, word: &str) -> Self {
        let mut word_array = [0u8; 64]; // 고정 크기 배열 초기화
        let bytes = word.as_bytes();

        // word 길이가 배열 크기를 초과하면 잘라냄
        let len = bytes.len().min(64);
        word_array[..len].copy_from_slice(&bytes[..len]);

        Self {
            column,
            word: word_array,
        }
    }

    pub fn get_word(&self) -> &str {
        // null 바이트 제거 후 문자열로 반환
        let len = self.word.iter().position(|&c| c == 0).unwrap_or(self.word.len());
        std::str::from_utf8(&self.word[..len]).unwrap()
    }

    pub fn set_word(&mut self, new_word: &str) {
        // 기존 word를 초기화
        self.word = [0u8; 64];
        let bytes = new_word.as_bytes();

        // 새 문자열을 복사
        let len = bytes.len().min(64);
        self.word[..len].copy_from_slice(&bytes[..len]);
    }
}