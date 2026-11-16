.PHONY = py

py:
	@black dedup.py
	@pylint --exit-zero dedup.py
	@mypy --strict dedup.py
