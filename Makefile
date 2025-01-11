# プロジェクト名
PROJECT_NAME := img_in_term

# バージョン
VERSION := 0.1.1

# インストール先
PREFIX := /usr/local

# ビルドターゲット
TARGET := target/release/$(PROJECT_NAME)

# manページディレクトリ
MANDIR := $(PREFIX)/share/man/man1

# リリースビルド
release:
        cargo build --release

# インストール
install: release
        install -Dm 755 $(TARGET) $(PREFIX)/bin/$(PROJECT_NAME)
        # manページがある場合
        @if [ -f $(PROJECT_NAME).1 ]; then \
                install -Dm 644 $(PROJECT_NAME).1 $(MANDIR)/$(PROJECT_NAME).1; \
        fi

# アンインストール
uninstall:
        rm -f $(PREFIX)/bin/$(PROJECT_NAME)
        rm -f $(MANDIR)/$(PROJECT_NAME).1

# クリーン
clean:
        cargo clean

# dist: tarballを作成
dist: release
        tar czvf $(PROJECT_NAME)-$(VERSION).tar.gz target/release/$(PROJECT_NAME) $(PROJECT_NAME).1

.PHONY: release install uninstall clean dist
