from abc import ABC, abstractmethod
from typing import Any, NamedTuple

from snowq.cursor import Cursor


class QueryParams(NamedTuple):
    query: str
    params: dict[str, Any] | tuple[dict[str, Any], ...]


class QueryBuilder(ABC):
    @abstractmethod
    def build(self) -> QueryParams: ...

    def execute(self, cursor: Cursor, /) -> None:
        query, params = self.build()

        if isinstance(params, tuple):
            cursor.executemany(query, params)
        else:
            cursor.execute(query, params)