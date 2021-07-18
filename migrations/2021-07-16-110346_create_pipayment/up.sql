create table pipayment (
  id bigserial primary key,
  person_id uuid,  
  payment_id uuid,   
  user_uid uuid,  
  person_name text,
  identifier text,
  amount double precision,
  memo text,
  to_address text,
  created_at timestamp,
  approved bool,
  verified bool,
  completed bool,
  cancelled bool,
  user_cancelled bool,
  tx_verified bool,
  tx_id text,  
  tx_link text,
  payment_dto jsonb
);
