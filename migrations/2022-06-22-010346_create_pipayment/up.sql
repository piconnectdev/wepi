create table pipayment (
  id uuid NOT NULL DEFAULT next_uuid(),
  domain text,
  instance_id uuid,
  person_id uuid,
  obj_cat text,
  obj_id uuid,
  a2u int default 0,
  asset text,  
  fee double precision,
  step int default 0,
  testnet bool,
  finished bool default false,
  published timestamp not null default now(),
  updated timestamp,
  ref_id uuid,
  comment text,  
  stat text,

  pi_uid uuid, 
  pi_username text,

  identifier text,
  user_uid text,
  amount double precision,
  memo text,
  from_address text,
  to_address text,
  direction text,
  created_at timestamp,
  network text,
  approved bool,
  verified bool,
  completed bool,
  cancelled bool,
  user_cancelled bool,
  tx_verified bool,
  tx_id text,  
  tx_link text,
  metadata jsonb,
  extras jsonb,
  primary key(id, published),
  CONSTRAINT pipayment_identifier_unique UNIQUE (identifier)
);

create index idx_pipayment_domain on pipayment (domain);
create index idx_pipayment_instance_id on pipayment (instance_id);
create index idx_pipayment_obj_cat on pipayment (obj_cat);
create index idx_pipayment_obj_id on pipayment (obj_id);
create index idx_pipayment_creator on pipayment (person_id);
create index idx_pipayment_pi_username on pipayment (pi_username);
create index idx_pipayment_pi_uid on pipayment (pi_uid);
create index idx_pipayment_user_uid on pipayment (user_uid);
create index idx_pipayment_identifier on pipayment (identifier);
create index idx_pipayment_memo on pipayment (memo);

create table person_balance (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid,
  person_name text,
  published timestamp not null default now(),
  asset text,
  stat text default 'active',
  deposited double precision,
  spent double precision,
  sent double precision,
  received double precision,
  withdrawed double precision,
  amount double precision,
  pending double precision,
  updated timestamp,
  extras jsonb,
  UNIQUE (person_id, asset)
);

-- create index idx_person_balance_person_id on person_balance (person_id);

create table person_withdraw (
  id uuid NOT NULL DEFAULT next_uuid() primary key,
  person_id uuid,
  person_name text,
  published timestamp not null default now(),
  asset_name text,
  finished bool,
  current_amount double precision,
  amount double precision,
  remain double precision,
  payment_id uuid,
  identifier text,
  stat text,
  txid text,
  link text,
  code text,
  notes text,
  updated timestamp,
  extras jsonb
);

create index idx_person_withdraw_person_id on person_withdraw (person_id);
create index idx_person_withdraw_code on person_withdraw (code);