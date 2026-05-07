.PHONY: test quick rust-test ts-test validate forbidden formal paper clean manifest ci-evidence docker-build docker-test

test:
	bash scripts/run_release_gates.sh

quick: validate forbidden ts-test formal

rust-test:
	cd rust && cargo test --all-features
	cd rust && cargo test --test negative_corpus
	cd rust && cargo test --test golden_vectors
	cd rust && cargo test --test proptest_invariants
	cd rust && cargo test --test memory_lineage_compression
	cd rust && cargo test --test numerical_stability
	cd rust && cargo test --test federation_quorum

ts-test:
	cd ts && npm test

validate:
	python3 tools/validate_vectors.py

forbidden:
	python3 tools/forbidden_imports.py

formal:
	bash scripts/run_formal_gates.sh

paper:
	cd paper && latexmk -pdf -interaction=nonstopmode main.tex

manifest:
	python3 tools/make_release_manifest.py

ci-evidence:
	bash scripts/collect_ci_evidence.sh

docker-build:
	docker build -t scc-kernel-fortress .

docker-test: docker-build
	docker run --rm scc-kernel-fortress

clean:
	find . -name '*.aux' -o -name '*.log' -o -name '*.out' -o -name '*.bbl' -o -name '*.blg' -o -name '*.fls' -o -name '*.fdb_latexmk' | xargs -r rm -f
	rm -rf validation
