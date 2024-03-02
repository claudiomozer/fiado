create table "user" (
	id uuid primary key not null,
	name varchar(255) not null,
	document char(11) not null unique,
	status varchar(255) not null,
	"password" varchar(255) not null,
	birth_date date not null,
	created_at timestamp not null,
	updated_at timestamp not null
);
