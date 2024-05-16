tailwind:
    tailwindcss -i ./input.css -o ./style/output.css --watch

build-raspi:
	LEPTOS_OUTPUT_NAME=calendula cargo build --target aarch64-unknown-linux-gnu --no-default-features --features ssr -r

deploy-raspi:
    scp target/aarch64-unknown-linux-gnu/release/calendula raspi_calendula:app

build-and-deploy-raspi: build-raspi deploy-raspi

