//go:build darwin

package pkg

/*
#cgo CFLAGS: -x objective-c -mmacosx-version-min=10.13
#cgo LDFLAGS: -framework Cocoa -framework AppKit
#import <Cocoa/Cocoa.h>
#import <Foundation/Foundation.h>

typedef struct {
    int x, y, w, h;
} ScreenRect;

CGFloat zMonScale(int idx) {
	NSArray<NSScreen*> *screens = [NSScreen screens];
	if(idx < 0 || idx >= (int)screens.count) return 1.0;
	return screens[idx].backingScaleFactor;
}
CGFloat zMenubarThickness() {
	return [NSStatusBar systemStatusBar].thickness;
}
int zIsPrimary(int idx) {
	NSArray<NSScreen*> *screens = [NSScreen screens];
	if(idx < 0 || idx >= (int)screens.count) return 0;
	NSScreen *main = [NSScreen mainScreen];
	return screens[idx] == main ? 1 : 0;
}

// Get mouse cursor position
ScreenRect zGetCursorLocation() {
    NSPoint location = [NSEvent mouseLocation];
    ScreenRect rect = {(int)location.x, (int)location.y, 0, 0};
    return rect;
}

// Get number of displays
int zGetDisplayCount() {
    NSArray<NSScreen*> *screens = [NSScreen screens];
    return (int)[screens count];
}

// Get display bounds for display at index i
ScreenRect zGetDisplayBounds(int index) {
    NSArray<NSScreen*> *screens = [NSScreen screens];
    if (index < 0 || index >= [screens count]) {
        ScreenRect empty = {0, 0, 0, 0};
        return empty;
    }

    NSScreen *screen = screens[index];
    NSRect frame = [screen frame];

    // Convert from Cocoa coordinates (bottom-left origin) to screen coordinates (top-left origin)
    NSScreen *mainScreen = [NSScreen mainScreen];
    CGFloat mainHeight = [mainScreen frame].size.height;

    ScreenRect rect = {
        (int)frame.origin.x,
        (int)(mainHeight - frame.origin.y - frame.size.height),
        (int)frame.size.width,
        (int)frame.size.height
    };
    return rect;
}
*/
import "C"

// MenuBarShiftPhysical returns the vertical pixel shift needed to place a window below the menubar.
// Returns 0 for non-primary displays or if unavailable.
func MenuBarShiftPhysical(index int) int {
	if C.zIsPrimary(C.int(index)) != 1 {
		return 0
	}
	mbDip := float64(C.zMenubarThickness())
	scale := float64(C.zMonScale(C.int(index)))
	if scale == 0 {
		scale = 1
	}
	return int(mbDip*scale + 0.5)
}

// ScreenRect represents a screen rectangle
type ScreenRect struct {
	X, Y, W, H int
}

// Location returns the current mouse cursor position
func Location() (int, int) {
	rect := C.zGetCursorLocation()
	return int(rect.x), int(rect.y)
}

// DisplaysNum returns the number of connected displays
func DisplaysNum() int {
	return int(C.zGetDisplayCount())
}

// GetDisplayBounds returns the bounds of the display at the given index
func GetDisplayBounds(index int) (int, int, int, int) {
	rect := C.zGetDisplayBounds(C.int(index))
	return int(rect.x), int(rect.y), int(rect.w), int(rect.h)
}

// GetScreenRect returns the screen rectangle for the display at the given index
func GetScreenRect(index int) ScreenRect {
	rect := C.zGetDisplayBounds(C.int(index))
	return ScreenRect{
		X: int(rect.x),
		Y: int(rect.y),
		W: int(rect.w),
		H: int(rect.h),
	}
}
