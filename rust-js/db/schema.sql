create table available_colors (
    id    integer primary key autoincrement,
    color text
);

create table blocks (
    id       integer primary key autoincrement,
    y        integer not null,
    x        integer not null,
    color_id integer not null,

    foreign key (color_id) references available_colors (id)
);

