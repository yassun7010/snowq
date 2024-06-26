from typing import TypedDict

import pytest
import snowflake.connector.cursor
import snowman
from pydantic import BaseModel
from pytest_mock import MockFixture
from snowman._features import USE_TURU
from snowman.relation.table import Table

if USE_TURU:
    import turu.snowflake  # type: ignore[import]


@pytest.fixture
def mock_snowflake_cursor(
    mocker: MockFixture,
) -> snowflake.connector.cursor.SnowflakeCursor:
    return mocker.MagicMock(spec=snowflake.connector.cursor.SnowflakeCursor)


@pytest.fixture
def mock_turu_snowflake_connection(
    mocker: MockFixture,
) -> "turu.snowflake.MockConnection":
    if USE_TURU:
        import turu.snowflake  # type: ignore[import]

        return turu.snowflake.MockConnection()

    else:
        return mocker.MagicMock(spec=snowflake.connector.connection.SnowflakeConnection)


class _UserInsertColumns(TypedDict):
    id: int
    name: str


class _UserUpdateColumns(TypedDict, total=False):
    id: int
    name: str


@snowman.table("database", "public", "users")
class User(BaseModel, Table[_UserInsertColumns, _UserUpdateColumns]):
    id: int
    name: str


@pytest.fixture
def user() -> User:
    user = User(id=1, name="Alice")

    return user
