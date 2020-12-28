CREATE TABLE public.m_products (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name text NOT NULL,
  price bigint NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp,
  is_deleted boolean NOT NULL DEFAULT 'f',
  CONSTRAINT m_products_pk PRIMARY KEY (id)
);

GRANT INSERT, SELECT, UPDATE, DELETE ON public.m_products TO aries;
