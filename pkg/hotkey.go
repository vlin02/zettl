package pkg

import (
	"encoding/json"

	hk "golang.design/x/hotkey"
)

type KeyboardEvent struct {
	Modifiers []string `json:"modifiers"`
	Code      string   `json:"code"`
}

var KeyToCode = map[hk.Key]string{
	hk.KeySpace:  "Space",
	hk.Key1:      "Digit1",
	hk.Key2:      "Digit2",
	hk.Key3:      "Digit3",
	hk.Key4:      "Digit4",
	hk.Key5:      "Digit5",
	hk.Key6:      "Digit6",
	hk.Key7:      "Digit7",
	hk.Key8:      "Digit8",
	hk.Key9:      "Digit9",
	hk.Key0:      "Digit0",
	hk.KeyA:      "KeyA",
	hk.KeyB:      "KeyB",
	hk.KeyC:      "KeyC",
	hk.KeyD:      "KeyD",
	hk.KeyE:      "KeyE",
	hk.KeyF:      "KeyF",
	hk.KeyG:      "KeyG",
	hk.KeyH:      "KeyH",
	hk.KeyI:      "KeyI",
	hk.KeyJ:      "KeyJ",
	hk.KeyK:      "KeyK",
	hk.KeyL:      "KeyL",
	hk.KeyM:      "KeyM",
	hk.KeyN:      "KeyN",
	hk.KeyO:      "KeyO",
	hk.KeyP:      "KeyP",
	hk.KeyQ:      "KeyQ",
	hk.KeyR:      "KeyR",
	hk.KeyS:      "KeyS",
	hk.KeyT:      "KeyT",
	hk.KeyU:      "KeyU",
	hk.KeyV:      "KeyV",
	hk.KeyW:      "KeyW",
	hk.KeyX:      "KeyX",
	hk.KeyY:      "KeyY",
	hk.KeyZ:      "KeyZ",
	hk.KeyReturn: "Enter",
	hk.KeyEscape: "Escape",
	hk.KeyDelete: "Backspace",
	hk.KeyTab:    "Tab",
	hk.KeyLeft:   "ArrowLeft",
	hk.KeyRight:  "ArrowRight",
	hk.KeyUp:     "ArrowUp",
	hk.KeyDown:   "ArrowDown",
	hk.KeyF1:     "F1",
	hk.KeyF2:     "F2",
	hk.KeyF3:     "F3",
	hk.KeyF4:     "F4",
	hk.KeyF5:     "F5",
	hk.KeyF6:     "F6",
	hk.KeyF7:     "F7",
	hk.KeyF8:     "F8",
	hk.KeyF9:     "F9",
	hk.KeyF10:    "F10",
	hk.KeyF11:    "F11",
	hk.KeyF12:    "F12",
	hk.KeyF13:    "F13",
	hk.KeyF14:    "F14",
	hk.KeyF15:    "F15",
	hk.KeyF16:    "F16",
	hk.KeyF17:    "F17",
	hk.KeyF18:    "F18",
	hk.KeyF19:    "F19",
	hk.KeyF20:    "F20",
}

var CodeToKey = map[string]hk.Key{
	"Space":      hk.KeySpace,
	"Digit1":     hk.Key1,
	"Digit2":     hk.Key2,
	"Digit3":     hk.Key3,
	"Digit4":     hk.Key4,
	"Digit5":     hk.Key5,
	"Digit6":     hk.Key6,
	"Digit7":     hk.Key7,
	"Digit8":     hk.Key8,
	"Digit9":     hk.Key9,
	"Digit0":     hk.Key0,
	"KeyA":       hk.KeyA,
	"KeyB":       hk.KeyB,
	"KeyC":       hk.KeyC,
	"KeyD":       hk.KeyD,
	"KeyE":       hk.KeyE,
	"KeyF":       hk.KeyF,
	"KeyG":       hk.KeyG,
	"KeyH":       hk.KeyH,
	"KeyI":       hk.KeyI,
	"KeyJ":       hk.KeyJ,
	"KeyK":       hk.KeyK,
	"KeyL":       hk.KeyL,
	"KeyM":       hk.KeyM,
	"KeyN":       hk.KeyN,
	"KeyO":       hk.KeyO,
	"KeyP":       hk.KeyP,
	"KeyQ":       hk.KeyQ,
	"KeyR":       hk.KeyR,
	"KeyS":       hk.KeyS,
	"KeyT":       hk.KeyT,
	"KeyU":       hk.KeyU,
	"KeyV":       hk.KeyV,
	"KeyW":       hk.KeyW,
	"KeyX":       hk.KeyX,
	"KeyY":       hk.KeyY,
	"KeyZ":       hk.KeyZ,
	"Enter":      hk.KeyReturn,
	"Escape":     hk.KeyEscape,
	"Backspace":  hk.KeyDelete,
	"Tab":        hk.KeyTab,
	"ArrowLeft":  hk.KeyLeft,
	"ArrowRight": hk.KeyRight,
	"ArrowUp":    hk.KeyUp,
	"ArrowDown":  hk.KeyDown,
	"F1":         hk.KeyF1,
	"F2":         hk.KeyF2,
	"F3":         hk.KeyF3,
	"F4":         hk.KeyF4,
	"F5":         hk.KeyF5,
	"F6":         hk.KeyF6,
	"F7":         hk.KeyF7,
	"F8":         hk.KeyF8,
	"F9":         hk.KeyF9,
	"F10":        hk.KeyF10,
	"F11":        hk.KeyF11,
	"F12":        hk.KeyF12,
	"F13":        hk.KeyF13,
	"F14":        hk.KeyF14,
	"F15":        hk.KeyF15,
	"F16":        hk.KeyF16,
	"F17":        hk.KeyF17,
	"F18":        hk.KeyF18,
	"F19":        hk.KeyF19,
	"F20":        hk.KeyF20,
}

var ModifierToString = map[hk.Modifier]string{
	hk.ModCtrl:   "Control",
	hk.ModShift:  "Shift",
	hk.ModOption: "Alt",
	hk.ModCmd:    "Meta",
}

var StringToModifier = map[string]hk.Modifier{
	"Control": hk.ModCtrl,
	"Shift":   hk.ModShift,
	"Alt":     hk.ModOption,
	"Meta":    hk.ModCmd,
}

type Hotkey struct {
	Mods []hk.Modifier
	Key  hk.Key
}

func NewHotkey(mods []hk.Modifier, key hk.Key) Hotkey {
	return Hotkey{Mods: mods, Key: key}
}

func (h Hotkey) Marshal() string {
	uintMods := make([]uint32, len(h.Mods))
	for i, m := range h.Mods {
		uintMods[i] = uint32(m)
	}
	data := struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}{Mods: uintMods, Key: uint32(h.Key)}
	jsonBytes, _ := json.Marshal(data)
	return string(jsonBytes)
}

func UnmarshalHotkey(s string) Hotkey {
	var data struct {
		Mods []uint32 `json:"mods"`
		Key  uint32   `json:"key"`
	}
	json.Unmarshal([]byte(s), &data)
	mods := make([]hk.Modifier, len(data.Mods))
	for i, m := range data.Mods {
		mods[i] = hk.Modifier(m)
	}
	return Hotkey{Mods: mods, Key: hk.Key(data.Key)}
}

func (h Hotkey) ToOpaque() *hk.Hotkey {
	return hk.New(h.Mods, h.Key)
}

func (h Hotkey) ToEvent() *KeyboardEvent {
	e := &KeyboardEvent{Modifiers: make([]string, 0, len(h.Mods)), Code: KeyToCode[h.Key]}
	for _, m := range h.Mods {
		if s, ok := ModifierToString[m]; ok {
			e.Modifiers = append(e.Modifiers, s)
		}
	}
	return e
}

func (e *KeyboardEvent) ToHotkey() Hotkey {
	k, ok := CodeToKey[e.Code]
	if !ok {
		panic("invalid code")
	}
	mods := make([]hk.Modifier, 0, len(e.Modifiers))
	for _, s := range e.Modifiers {
		m, ok := StringToModifier[s]
		if !ok {
			panic("invalid modifier")
		}
		mods = append(mods, m)
	}
	return Hotkey{Mods: mods, Key: k}
}
