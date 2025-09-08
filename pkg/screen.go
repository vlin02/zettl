package pkg

import (
	robotgo "github.com/go-vgo/robotgo"
)

type Rect struct {
	X int `json:"x"`
	Y int `json:"y"`
	W int `json:"w"`
	H int `json:"h"`
}

func GetCurrentDisplayId() int {
	mouseX, mouseY := robotgo.Location()
	numDisplays := robotgo.DisplaysNum()
	for i := range numDisplays {
		x, y, w, h := robotgo.GetDisplayBounds(i)
		if mouseX >= x && mouseX < x+w && mouseY >= y && mouseY < y+h {
			return i
		}
	}
	return 0
}

func GetCurrentScreenRect() Rect {
	displayId := GetCurrentDisplayId()
	rect := robotgo.GetScreenRect(displayId)
	y := rect.Y
	h := rect.H

	if displayId == 0 {
		y += 25
		h -= 25
	}

	return Rect{X: rect.X, Y: y, W: rect.W, H: h}
}
