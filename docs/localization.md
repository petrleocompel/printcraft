# Interface language

Choose **Edit > Preferences > Interface language** and select **English**, **日本語** (Japanese) or **Čeština** (Czech). The change applies immediately and persists between launches. Command ids, document contents and file names are unchanged.

The control channel exposes the setting through `ui.set`:

```json
{"method":"ui.set","params":{"key":"language","value":"cs"}}
```

`ui.state` reports `language` as `en`, `ja` or `cs`. Unknown language codes return an error without changing the current setting. Old preferences default to English.

The translations cover the main menu: the File, Edit, Pages, View and Help titles, every registered command shown in those menus, the zoom, theme and side-panel entries routed through the translation table, and the Preferences dialog. Czech covers all of them, and a test fails when a menu command or a translated label is added without a Czech entry. Japanese covers the core commands. Labels without a translation, including dialogs, panels and the "Undo …"/"Redo …" labels that name the last edit, use English. Vertical Japanese PDF rendering is an existing viewer feature; this change does not add vertical text editing.
