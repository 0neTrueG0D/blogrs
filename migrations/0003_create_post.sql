CREATE TABLE IF NOT EXISTS post (
    id SERIAL PRIMARY KEY, 
    title VARCHAR(255) NOT NULL, 
    slug VARCHAR(255) NOT NULL UNIQUE, 
    user_id INT NOT NULL,
    excerpt TEXT NOT NULL, 
    content TEXT NOT NULL, 
    category_id INT, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(), 
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_users FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_category FOREIGN KEY(category_id) REFERENCES category(id) ON DELETE SET NULL
);