//go:build darwin

package pkg

/*
#cgo CFLAGS: -x objective-c -mmacosx-version-min=10.13
#cgo LDFLAGS: -framework Cocoa -framework AppKit
#import <Cocoa/Cocoa.h>
#import <Foundation/Foundation.h>

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
