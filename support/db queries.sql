
delete from activity where id = '8c5347f5-88df-4f5e-bba2-a9e2712ad997'


select t1.*, t3.* from activity t1, activity_tag t2, tag t3
WHERE t1.id = t2.activity_id AND t2.tag_id = t3.id;


select * from monetary_medium;

select * from tag

select * from activity_tag

select * from activity

COMMIT;


SELECT a.id, a.value, a.medium_id, a.operation, a.description, a.date, 
                m.name as medium_name, m.is_valid_for_credit 
         FROM activity a 
         JOIN monetary_medium m ON a.medium_id = m.id



INSERT INTO activity_tag
VALUES
('fe0e633a-d8db-4950-9174-777ca66846a0', 2),
('fe0e633a-d8db-4950-9174-777ca66846a0', 4)



INSERT INTO tag
VALUES
(1, 'mercado'),
(2, 'combustivel'),
(3, 'aluguel'),
(4, 'internet')



INSERT INTO monetary_medium
VALUES 
(1, 'Crédito Santander', false),
(2, 'Crédito Nubank', false),
(3, 'Crédito Neon', false),
(4, 'Débito Santander', false),
(5, 'Débito Nubank', false),
(6, 'PIX Santander', true),
(7, 'PIX Nubank', true),
(8, 'Boleto Santander', false),
(9, 'Dinheiro (wtf)', true)


