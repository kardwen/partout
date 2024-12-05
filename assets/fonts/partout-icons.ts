export type PartoutIconsId =
  | "clipboard"
  | "document"
  | "key"
  | "refresh"
  | "brush"
  | "eye-close"
  | "info"
  | "file"
  | "book"
  | "chronometer"
  | "alert"
  | "search"
  | "settings-alt"
  | "log-in"
  | "eye";

export type PartoutIconsKey =
  | "Clipboard"
  | "Document"
  | "Key"
  | "Refresh"
  | "Brush"
  | "EyeClose"
  | "Info"
  | "File"
  | "Book"
  | "Chronometer"
  | "Alert"
  | "Search"
  | "SettingsAlt"
  | "LogIn"
  | "Eye";

export enum PartoutIcons {
  Clipboard = "clipboard",
  Document = "document",
  Key = "key",
  Refresh = "refresh",
  Brush = "brush",
  EyeClose = "eye-close",
  Info = "info",
  File = "file",
  Book = "book",
  Chronometer = "chronometer",
  Alert = "alert",
  Search = "search",
  SettingsAlt = "settings-alt",
  LogIn = "log-in",
  Eye = "eye",
}

export const PARTOUT_ICONS_CODEPOINTS: { [key in PartoutIcons]: string } = {
  [PartoutIcons.Clipboard]: "61697",
  [PartoutIcons.Document]: "61698",
  [PartoutIcons.Key]: "61699",
  [PartoutIcons.Refresh]: "61700",
  [PartoutIcons.Brush]: "61701",
  [PartoutIcons.EyeClose]: "61702",
  [PartoutIcons.Info]: "61703",
  [PartoutIcons.File]: "61704",
  [PartoutIcons.Book]: "61705",
  [PartoutIcons.Chronometer]: "61706",
  [PartoutIcons.Alert]: "61707",
  [PartoutIcons.Search]: "61708",
  [PartoutIcons.SettingsAlt]: "61709",
  [PartoutIcons.LogIn]: "61710",
  [PartoutIcons.Eye]: "61711",
};
