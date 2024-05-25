import textwrap

import snowq
from conftest import User
from snowflake.connector.cursor import SnowflakeCursor


class TestUpdateQuery:
    def test_update_query(self, user: User, mock_snowflake_cursor: SnowflakeCursor):
        snowq.query.update(User).set({"id": 1, "name": "taro"}).execute(
            mock_snowflake_cursor
        )

    def test_update_query_dict_build(
        self, user: User, mock_snowflake_cursor: SnowflakeCursor
    ):
        query, params = snowq.query.update(User).set({"name": "taro"}).build()

        assert (
            query
            == textwrap.dedent(
                """
                UPDATE
                    database.public.users
                SET
                    name = %(name)s
                """
            ).strip()
        )
        assert params == {"name": "taro"}

    def test_update_query_pydantic_build(self, user: User):
        query, params = snowq.query.update(User).set(user).build()

        assert (
            query
            == textwrap.dedent(
                """
                UPDATE
                    database.public.users
                SET
                    id = %(id)s,
                    name = %(name)s
                """
            ).strip()
        )
        assert params == user.model_dump()