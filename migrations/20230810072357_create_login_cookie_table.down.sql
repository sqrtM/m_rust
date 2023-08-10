ALTER TABLE login_cookies
    DROP CONSTRAINT fk_cookie_to_user;

DROP TABLE login_cookies;