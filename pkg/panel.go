//go:build darwin

package pkg

/*
#cgo CFLAGS: -x objective-c -mmacosx-version-min=10.13
#cgo LDFLAGS: -framework Cocoa -framework AppKit
#import <Cocoa/Cocoa.h>
#import <Foundation/Foundation.h>

static NSScreen *screenUnderMouse(void) {
    NSPoint p = [NSEvent mouseLocation];
    for (NSScreen *s in [NSScreen screens]) {
        if (NSPointInRect(p, s.frame))
            return s;
    }
    return [NSScreen mainScreen];
}

static void placeOnCursorScreen(NSWindow *w) {
    NSScreen *s = screenUnderMouse();
    NSRect vis = s.visibleFrame;
    CGFloat marginTop = 50;
    CGFloat marginBottom = 50;
    CGFloat w0 = w.frame.size.width;
    CGFloat h0 = vis.size.height - marginTop - marginBottom;
    CGFloat x = vis.origin.x + (vis.size.width - w0) * 0.5;
    CGFloat y = vis.origin.y + marginBottom;
    [w setFrame:(NSRect){{x, y}, {w0, h0}} display:NO];
}

void zShowPanel(void *window, bool hide) {
    NSWindow *w = (NSWindow *)window;
    if (hide) {
        [NSApp hide:nil];
    }
    dispatch_async(dispatch_get_main_queue(), ^{
        placeOnCursorScreen(w);
        [w orderFrontRegardless];
        dispatch_async(dispatch_get_main_queue(), ^{
            [w makeKeyWindow];
        });
    });
}

void zHidePanelOnResignKey(void *window) {
    NSWindow *w = (NSWindow *)window;
    [NSNotificationCenter.defaultCenter
        addObserverForName:NSWindowDidResignKeyNotification
                    object:w
                     queue:NSOperationQueue.mainQueue
                usingBlock:^(__unused NSNotification *n) {
                    [w orderOut:nil];
                }];
}

*/
import "C"
import "unsafe"

// ShowPanel shows the panel window positioned at the cursor's screen
func ShowPanel(window unsafe.Pointer, hide bool) {
	C.zShowPanel(window, C.bool(hide))
}

// HidePanelOnResignKey automatically hides the panel when it loses key focus
func HidePanelOnResignKey(window unsafe.Pointer) {
	C.zHidePanelOnResignKey(window)
}
