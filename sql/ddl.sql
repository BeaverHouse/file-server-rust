CREATE TABLE family_app.alarms (
	id serial4 NOT NULL,
	user_id varchar(255) NOT NULL,
	file_path varchar(255) NOT NULL,
	delete_flag varchar(1) NOT NULL,
	CONSTRAINT alarms_pkey PRIMARY KEY (id)
);

CREATE TABLE family_app.users (
	id serial4 NOT NULL,
	user_id varchar(255) NOT NULL,
	"name" varchar(255) NOT NULL,
	CONSTRAINT users_pkey PRIMARY KEY (id),
	CONSTRAINT users_user_id_key UNIQUE (user_id)
);