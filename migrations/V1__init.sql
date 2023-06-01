CREATE TABLE profile (
    id BIGINT PRIMARY KEY,
    name TEXT,
    meta JSONB,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);
CREATE TABLE word (
    id BIGSERIAL,
    word TEXT NOT NULL,
    meta JSONB,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);
CREATE TABLE meaning (
    id BIGSERIAL,
    word_id BIGINT,
    part_of_speech TEXT NOT NULL,
    synonyms TEXT [],
    antonyms TEXT [],
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);
CREATE TABLE definition (
    id BIGSERIAL,
    meaning_id BIGINT,
    definition TEXT NOT NULL,
    synonyms TEXT [],
    antonyms TEXT []
);
