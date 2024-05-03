from abc import ABC, abstractmethod
from typing import NamedTuple

from snowman.protocol.cursor import Cursor


class QueryParams(NamedTuple):
    query: str
    params: dict[str, str]


class QueryBuilder(ABC):
    @abstractmethod
    def build(self) -> QueryParams: ...

    def execute(self, cursor: Cursor) -> None:
        query, params = self.build()
        cursor.execute(query, params)
