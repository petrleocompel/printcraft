//! Interface translations. Command ids, document text and file names remain stable.
//! Untranslated labels fall back to English so coverage can grow incrementally.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
    Cs,
}

impl Language {
    pub const ALL: [Self; 3] = [Self::En, Self::Ja, Self::Cs];

    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ja => "日本語",
            Self::Cs => "Čeština",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "ja" => Some(Self::Ja),
            "cs" => Some(Self::Cs),
            _ => None,
        }
    }

    /// The (English, translation) pairs for this language; empty for English.
    pub(crate) fn table(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::En => &[],
            Self::Ja => JAPANESE,
            Self::Cs => CZECH,
        }
    }

    pub fn tr(self, text: &str) -> &str {
        self.table().iter().find(|(english, _)| *english == text).map_or(text, |(_, translated)| translated)
    }
}

const JAPANESE: &[(&str, &str)] = &[
    ("Menu", "メニュー"),
    ("File", "ファイル"),
    ("Edit", "編集"),
    ("Pages", "ページ"),
    ("View", "表示"),
    ("Help", "ヘルプ"),
    ("Preferences", "環境設定"),
    ("Preferences…", "環境設定…"),
    ("Interface language", "表示言語"),
    ("Identity", "個人情報"),
    ("Name on new comments", "新しい注釈の作成者名"),
    ("Open…", "開く…"),
    ("New blank PDF", "空白の PDF を作成"),
    ("Create PDF from file…", "ファイルから PDF を作成…"),
    ("Create PDF from images…", "画像から PDF を作成…"),
    ("Create PDF from clipboard", "クリップボードから PDF を作成"),
    ("Combine files…", "ファイルを結合…"),
    ("Save", "保存"),
    ("Save as…", "別名で保存…"),
    ("Close file", "ファイルを閉じる"),
    ("Close all", "すべて閉じる"),
    ("Revert", "保存済みの状態に戻す"),
    ("Print…", "印刷…"),
    ("Document properties…", "文書のプロパティ…"),
    ("Undo", "取り消し"),
    ("Redo", "やり直し"),
    ("Find…", "検索…"),
    ("Advanced search…", "高度な検索…"),
    ("Copy pages", "ページをコピー"),
    ("Cut pages", "ページを切り取り"),
    ("Paste pages", "ページを貼り付け"),
    ("Fit visible", "表示範囲に合わせる"),
    ("Marquee zoom", "範囲指定ズーム"),
    ("Take a snapshot", "スナップショットを作成"),
    ("Full screen mode", "全画面表示"),
    ("Read mode", "閲覧モード"),
    ("Switch light / dark theme", "明るい／暗いテーマを切り替え"),
    ("Comments panel", "コメントパネル"),
    ("Form fields panel", "フォームフィールドパネル"),
    ("Clear form", "フォームをクリア"),
    ("Find tools and commands…", "ツールとコマンドを検索…"),
    ("Zoom", "ズーム"),
    ("Actual size", "実際のサイズ"),
    ("Zoom to page level", "ページ全体を表示"),
    ("Fit to width", "幅に合わせる"),
    ("Display theme", "表示テーマ"),
    ("Side panels", "サイドパネル"),
    ("Enable Acrobat JavaScript", "Acrobat JavaScript を有効にする"),
    ("OK", "OK"),
];

/// Czech (Čeština). Written clean-room from the meaning of the English labels, following
/// common Czech desktop conventions (menus are nouns, commands are infinitives); it is not
/// taken from Adobe Acrobat's Czech localisation or any other product's translation.
/// Terminology: annotations are "komentáře", pages are "stránky", form fields are
/// "formulářová pole". Product and format names (PrintCraft, PDF, Word, HTML…) stay in Latin.
const CZECH: &[(&str, &str)] = &[
    // Main menu, View submenus and Preferences.
    ("Menu", "Nabídka"),
    ("File", "Soubor"),
    ("Edit", "Úpravy"),
    ("Pages", "Stránky"),
    ("View", "Zobrazení"),
    ("Help", "Nápověda"),
    ("Zoom", "Zvětšení"),
    ("Actual size", "Skutečná velikost"),
    ("Zoom to page level", "Celá stránka"),
    ("Fit to width", "Přizpůsobit šířce"),
    ("Display theme", "Motiv vzhledu"),
    ("Side panels", "Postranní panely"),
    ("Preferences", "Předvolby"),
    ("Preferences…", "Předvolby…"),
    ("Interface language", "Jazyk rozhraní"),
    ("Identity", "Identita"),
    ("Name on new comments", "Jméno autora nových komentářů"),
    ("Enable Acrobat JavaScript", "Povolit Acrobat JavaScript"),
    ("OK", "OK"),
    // File.
    ("Open…", "Otevřít…"),
    ("New blank PDF", "Nový prázdný dokument PDF"),
    ("Create PDF from file…", "Vytvořit PDF ze souboru…"),
    ("Create PDF from images…", "Vytvořit PDF z obrázků…"),
    ("Create PDF from clipboard", "Vytvořit PDF ze schránky"),
    ("Combine files…", "Sloučit soubory…"),
    ("Save", "Uložit"),
    ("Save as…", "Uložit jako…"),
    ("Close file", "Zavřít soubor"),
    ("Close all", "Zavřít vše"),
    ("Revert", "Vrátit k uložené verzi"),
    ("Print…", "Tisk…"),
    ("Document properties…", "Vlastnosti dokumentu…"),
    ("Export to image…", "Exportovat jako obrázek…"),
    ("Reduce file size…", "Zmenšit velikost souboru…"),
    ("Optimize PDF…", "Optimalizovat PDF…"),
    ("Export to text…", "Exportovat jako text…"),
    ("Export to Word…", "Exportovat do formátu Word…"),
    ("Export to HTML…", "Exportovat do formátu HTML…"),
    ("Export to RTF…", "Exportovat do formátu RTF…"),
    ("Export all images…", "Exportovat všechny obrázky…"),
    ("Protect using password…", "Chránit heslem…"),
    ("Remove security", "Odebrat zabezpečení"),
    ("Security properties…", "Vlastnosti zabezpečení…"),
    // Edit.
    ("Undo", "Zpět"),
    ("Redo", "Znovu"),
    ("Find…", "Najít…"),
    ("Advanced search…", "Rozšířené hledání…"),
    ("Take a snapshot", "Pořídit snímek"),
    ("Clear form", "Vymazat formulář"),
    ("Copy pages", "Kopírovat stránky"),
    ("Cut pages", "Vyjmout stránky"),
    ("Paste pages", "Vložit stránky"),
    // View.
    ("Find tools and commands…", "Najít nástroje a příkazy…"),
    ("Fit visible", "Přizpůsobit viditelnému obsahu"),
    ("Marquee zoom", "Zvětšit vybranou oblast"),
    ("Full screen mode", "Režim celé obrazovky"),
    ("Read mode", "Režim čtení"),
    ("Switch light / dark theme", "Přepnout světlý / tmavý motiv"),
    ("Comments panel", "Panel komentářů"),
    ("Form fields panel", "Panel formulářových polí"),
    ("Signatures panel", "Panel podpisů"),
    // Pages.
    ("Organize pages", "Uspořádat stránky"),
    ("New bookmark", "Nová záložka"),
    ("Rotate pages clockwise", "Otočit stránky po směru hodinových ručiček"),
    ("Rotate pages counterclockwise", "Otočit stránky proti směru hodinových ručiček"),
    ("Delete pages", "Odstranit stránky"),
    ("Insert blank page", "Vložit prázdnou stránku"),
    ("Rotate pages…", "Otočit stránky…"),
    ("Duplicate pages", "Duplikovat stránky"),
    ("Crop pages", "Oříznout stránky"),
    ("Set page boxes…", "Nastavit rámečky stránek…"),
    ("Insert pages from file…", "Vložit stránky ze souboru…"),
    ("Replace pages…", "Nahradit stránky…"),
    ("Extract pages…", "Extrahovat stránky…"),
    ("Split document…", "Rozdělit dokument…"),
    ("Number pages…", "Očíslovat stránky…"),
    // Help.
    ("Keyboard shortcuts", "Klávesové zkratky"),
    ("Join the ArtCraft Discord", "Připojit se ke komunitě ArtCraft na Discordu"),
    ("PrintCraft web page", "Webová stránka PrintCraft"),
    ("PrintCraft on GitHub", "PrintCraft na GitHubu"),
    ("ArtCraft website", "Web ArtCraft"),
    ("Check for updates…", "Zkontrolovat aktualizace…"),
    ("About PrintCraft", "O aplikaci PrintCraft"),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Translations that are deliberately identical to the English source.
    const KEEP_AS_IS: &[&str] = &["OK"];

    /// Every string literal passed to `.tr("…")` in this crate's sources (outside this file).
    fn tr_literals() -> Vec<(std::path::PathBuf, String)> {
        let mut out = Vec::new();
        let mut dirs = vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    dirs.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "rs") || path.file_name().is_some_and(|n| n == "i18n.rs") {
                    continue;
                }
                let source = std::fs::read_to_string(&path).unwrap();
                for (at, open) in source.match_indices(".tr(\"") {
                    let literal = source[at + open.len()..].split('"').next().unwrap();
                    out.push((path.clone(), literal.to_string()));
                }
            }
        }
        out
    }

    #[test]
    fn translations_are_unique_and_preserve_unknown_text() {
        for language in Language::ALL {
            let table = language.table();
            for (i, (en, translated)) in table.iter().enumerate() {
                assert!(!translated.is_empty(), "{language:?}: empty translation for {en:?}");
                assert!(table.iter().take(i).all(|(other, _)| en != other), "{language:?}: duplicate key {en:?}");
                assert_eq!(Language::En.tr(en), *en);
                assert_eq!(language.tr(en), *translated);
            }
        }
        assert_eq!(Language::Ja.tr("File"), "ファイル");
        assert_eq!(Language::Cs.tr("File"), "Soubor");
        assert_eq!(Language::Ja.tr("日本語の文書.pdf"), "日本語の文書.pdf");
        assert_eq!(Language::Cs.tr("Žluťoučký kůň.pdf"), "Žluťoučký kůň.pdf");
        assert_eq!(Language::parse("xx"), None);
        for language in Language::ALL {
            let code = serde_json::to_value(language).unwrap();
            assert_eq!(Language::parse(code.as_str().unwrap()), Some(language));
        }
        assert_eq!(serde_json::to_value(Language::Cs).unwrap(), "cs");
        assert_eq!(Language::Cs.name(), "Čeština");
    }

    #[test]
    fn czech_covers_every_menu_label_and_tr_literal() {
        for spec in printcraft_engine::commands::COMMANDS {
            let Some(menu) = spec.menu else { continue };
            assert!(CZECH.iter().any(|(en, _)| *en == menu), "Czech lacks the menu title {menu:?}");
            assert!(CZECH.iter().any(|(en, _)| *en == spec.label), "Czech lacks the {menu} menu label {:?} ({})", spec.label, spec.id);
        }
        let literals = tr_literals();
        assert!(literals.len() >= 16, "the .tr(\"…\") scan found only {} call sites", literals.len());
        for (path, literal) in &literals {
            assert!(CZECH.iter().any(|(en, _)| en == literal), "Czech lacks {literal:?} from {}", path.display());
        }
        for (en, _) in JAPANESE {
            assert!(CZECH.iter().any(|(cs, _)| cs == en), "Czech lacks {en:?}, which Japanese translates");
        }
    }

    #[test]
    fn czech_translations_are_translated_and_keep_ellipses() {
        for keep in KEEP_AS_IS {
            assert_eq!(Language::Cs.tr(keep), *keep, "{keep:?} is listed as kept but missing or changed");
        }
        for (en, cs) in CZECH {
            assert!(en != cs || KEEP_AS_IS.contains(en), "{en:?} is untranslated; translate it or add it to KEEP_AS_IS");
            assert_eq!(en.ends_with('…'), cs.ends_with('…'), "ellipsis mismatch: {en:?} -> {cs:?}");
            assert!(!cs.contains("..."), "use … rather than three dots: {cs:?}");
            assert_eq!(cs.trim(), *cs, "stray whitespace: {cs:?}");
        }
    }

    #[test]
    fn language_persists_and_invalid_input_keeps_current_language() {
        let mut app = crate::PrintCraftApp::default();
        app.set_option("language", "ja").unwrap();
        assert_eq!(app.language, Language::Ja);
        assert!(app.set_option("language", "xx").is_err());
        assert_eq!(app.language, Language::Ja);
        let mut restored = crate::PrintCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::Ja);
        restored.restore(r#"{"language":"xx"}"#);
        assert_eq!(restored.language, Language::Ja);
        let mut legacy = crate::PrintCraftApp::default();
        legacy.restore("{}");
        assert_eq!(legacy.language, Language::En);

        use crate::control::Host;
        let mut app = crate::PrintCraftApp::default();
        Host::set(&mut app, "language", "cs").unwrap();
        assert_eq!(app.language, Language::Cs);
        assert_eq!(Host::state(&app)["language"], "cs");
        assert!(Host::set(&mut app, "language", "cz").is_err());
        assert_eq!(app.language, Language::Cs);
        let mut restored = crate::PrintCraftApp::default();
        restored.restore(&app.persist());
        assert_eq!(restored.language, Language::Cs);
        restored.restore(r#"{"language":"xx"}"#);
        assert_eq!(restored.language, Language::Cs);
    }
}
