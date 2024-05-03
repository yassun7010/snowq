import snowman
from conftest import User
from snowflake.connector.cursor import SnowflakeCursor


class TestUpdateStatement:
    def test_update_statement(self, user: User, mock_snowflake_cursor: SnowflakeCursor):
        snowman.query.update(User).set(user).execute(mock_snowflake_cursor)
