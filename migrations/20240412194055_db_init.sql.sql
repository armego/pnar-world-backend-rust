-----------------------------------------------------------------
-- Description: Basic migration that creates roles for the users
-- of the application and the basic user schema.
--
-- @author: Stavros Grigoriou <unix121@protonmail.com>
-----------------------------------------------------------------

-------------------
-- Table: user_role
-------------------
CREATE TABLE IF NOT EXISTS user_role
(
    role_id    VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT user_role_role_id_pk PRIMARY KEY (role_id)
);

COMMENT ON TABLE user_role IS 'Available roles of the application';
COMMENT ON COLUMN user_role.role_id IS 'The role name - pk';
COMMENT ON COLUMN user_role.created_at IS 'The date the role was created';

-- Insert the basic role: USER
INSERT INTO user_role (role_id)
VALUES ('admin'), ('moderator'), ('translator'), ('contributor'), ('user')
ON CONFLICT (role_id) DO NOTHING;

------------------
-- Table: users
------------------
-- This migration creates the IAM service tables
-- Note: Database creation is handled by setup_db.sh script
-- Drop and recreate the database for a fresh start
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    full_name VARCHAR(100),
    avatar_url TEXT,
    role VARCHAR(20) NOT NULL DEFAULT 'user' REFERENCES user_role(role_id) ON DELETE CASCADE,
    translation_points INTEGER NOT NULL DEFAULT 0,
    bio TEXT,
    preferred_language VARCHAR(10) NOT NULL DEFAULT 'en',
    settings JSONB DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add indexes for new fields
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_is_active ON users(is_active);
CREATE INDEX IF NOT EXISTS idx_users_translation_points ON users(translation_points);

-- Create translation tables for the translation service
CREATE TABLE IF NOT EXISTS pnar_dictionary (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pnar_word VARCHAR(255) NOT NULL UNIQUE,
    english_word VARCHAR(255) NOT NULL,
    part_of_speech VARCHAR(50),
    definition TEXT,
    example_pnar TEXT,
    example_english TEXT,
    difficulty_level INTEGER DEFAULT 1,
    usage_frequency INTEGER DEFAULT 0,
    cultural_context TEXT,
    related_words TEXT,
    pronunciation VARCHAR(255),
    etymology TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    verified BOOLEAN NOT NULL DEFAULT false,
    verified_by UUID REFERENCES users(id),
    verified_at TIMESTAMPTZ
);

-- Create translation requests table
CREATE TABLE IF NOT EXISTS translation_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    source_text TEXT NOT NULL,
    source_language VARCHAR(10) NOT NULL DEFAULT 'en',
    target_language VARCHAR(10) NOT NULL DEFAULT 'pnar',
    translated_text TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    translation_type VARCHAR(50) NOT NULL DEFAULT 'automatic',
    confidence_score DECIMAL(3,2),
    reviewed BOOLEAN NOT NULL DEFAULT false,
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create user contributions table
CREATE TABLE IF NOT EXISTS user_contributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    contribution_type VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    previous_value JSONB,
    new_value JSONB,
    points_awarded INTEGER DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create word usage analytics table
CREATE TABLE IF NOT EXISTS word_usage_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    word_id UUID NOT NULL REFERENCES pnar_dictionary(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    usage_type VARCHAR(50) NOT NULL,
    context_data JSONB DEFAULT '{}',
    session_id VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    data JSONB DEFAULT '{}',
    read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- Add indexes for performance
CREATE INDEX IF NOT EXISTS idx_pnar_dictionary_pnar_word ON pnar_dictionary(pnar_word);
CREATE INDEX IF NOT EXISTS idx_pnar_dictionary_english_word ON pnar_dictionary(english_word);
CREATE INDEX IF NOT EXISTS idx_pnar_dictionary_verified ON pnar_dictionary(verified);
CREATE INDEX IF NOT EXISTS idx_pnar_dictionary_difficulty ON pnar_dictionary(difficulty_level);
CREATE INDEX IF NOT EXISTS idx_pnar_dictionary_frequency ON pnar_dictionary(usage_frequency);

CREATE INDEX IF NOT EXISTS idx_translation_requests_user_id ON translation_requests(user_id);
CREATE INDEX IF NOT EXISTS idx_translation_requests_status ON translation_requests(status);
CREATE INDEX IF NOT EXISTS idx_translation_requests_created_at ON translation_requests(created_at);

CREATE INDEX IF NOT EXISTS idx_user_contributions_user_id ON user_contributions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_contributions_type ON user_contributions(contribution_type);
CREATE INDEX IF NOT EXISTS idx_user_contributions_status ON user_contributions(status);

CREATE INDEX IF NOT EXISTS idx_word_usage_word_id ON word_usage_analytics(word_id);
CREATE INDEX IF NOT EXISTS idx_word_usage_user_id ON word_usage_analytics(user_id);
CREATE INDEX IF NOT EXISTS idx_word_usage_created_at ON word_usage_analytics(created_at);

CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_read ON notifications(read);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add triggers for updated_at columns
CREATE TRIGGER update_pnar_dictionary_updated_at 
    BEFORE UPDATE ON pnar_dictionary 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_translation_requests_updated_at 
    BEFORE UPDATE ON translation_requests 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for the dictionary
INSERT INTO pnar_dictionary (id, pnar_word, english_word, part_of_speech, definition, example_pnar, example_english, difficulty_level, usage_frequency, verified)
VALUES 
    (gen_random_uuid(), 'ka', 'I/me', 'pronoun', 'First person singular pronoun', 'Ka phi Shillong', 'I go to Shillong', 1, 100, true),
    (gen_random_uuid(), 'phi', 'go', 'verb', 'To move from one place to another', 'Ka phi bazar', 'I go to market', 1, 95, true),
    (gen_random_uuid(), 'jong', 'house', 'noun', 'A building for human habitation', 'Jong ka ki ka', 'This is my house', 1, 90, true),
    (gen_random_uuid(), 'kaba', 'what', 'interrogative', 'Used to ask for information', 'Kaba ka ym ki?', 'What are you doing?', 1, 85, true),
    (gen_random_uuid(), 'kumno', 'how', 'interrogative', 'In what way or manner', 'Kumno ka phi?', 'How do I go?', 1, 80, true)
ON CONFLICT (pnar_word) DO NOTHING;

-- Insert dummy users for testing
-- Password for all test users is: password123
-- Hash generated with bcrypt cost 12: $2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW
-- INSERT INTO users (id, email, password, full_name, is_active, is_email_verified) VALUES
-- (gen_random_uuid(), 'admin@example.com', '$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW', 'System Administrator', true, true),
-- (gen_random_uuid(), 'john.doe@example.com', '$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW', 'John Doe', true, true),
-- (gen_random_uuid(), 'jane.smith@example.com', '$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW', 'Jane Smith', true, true),
-- (gen_random_uuid(), 'bob.wilson@example.com', '$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW', 'Bob Wilson', true, false),
-- (gen_random_uuid(), 'alice.brown@example.com', '$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW', 'Alice Brown', false, false)
-- ON CONFLICT (email) DO NOTHING;


