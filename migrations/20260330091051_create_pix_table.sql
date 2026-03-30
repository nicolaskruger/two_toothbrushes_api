-- Add migration script here
-- Criar enum para status do Pix
CREATE TYPE pix_status AS ENUM ('PENDING', 'PAID');

-- Tabela de pix
CREATE TABLE pix (
    id UUID PRIMARY KEY,
    amount DOUBLE PRECISION NOT NULL,
    qr_code TEXT NOT NULL,
    qr_code_base64 TEXT NOT NULL,
    group_id UUID NOT NULL,
    status pix_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_pix_group
        FOREIGN KEY (group_id)
        REFERENCES groups(id)
        ON DELETE CASCADE
);
