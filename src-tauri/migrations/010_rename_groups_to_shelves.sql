-- グループをシェルフに名称変更するマイグレーション

-- groupsテーブルをshelvesにリネーム
ALTER TABLE groups RENAME TO shelves;

-- active_groupテーブルをactive_shelfにリネーム
ALTER TABLE active_group RENAME TO active_shelf;

-- active_shelfテーブルのカラム名を変更
ALTER TABLE active_shelf RENAME COLUMN group_id TO shelf_id;