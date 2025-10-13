//go:build darwin

package pkg

/*
#cgo CFLAGS: -x objective-c -mmacosx-version-min=10.13
#cgo LDFLAGS: -framework Cocoa -framework AppKit -framework ApplicationServices -framework CoreGraphics
#import <Cocoa/Cocoa.h>
#import <Foundation/Foundation.h>
#import <ApplicationServices/ApplicationServices.h>
#import <CoreGraphics/CoreGraphics.h>

static _Atomic _Bool showPanelOnResign = 0;

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

void zShowPanel(void *window) {
    NSWindow *w = (NSWindow *)window;
    dispatch_async(dispatch_get_main_queue(), ^{
        placeOnCursorScreen(w);
        [w orderFrontRegardless];
        dispatch_async(dispatch_get_main_queue(), ^{
            [w makeKeyWindow];
        });

    });
}

void zSetupPanelNotifications(void *window) {
    NSWindow *w = (NSWindow *)window;

    [NSNotificationCenter.defaultCenter
        addObserverForName:NSApplicationDidBecomeActiveNotification
                    object:NSApp
                     queue:NSOperationQueue.mainQueue
                usingBlock:^(__unused NSNotification *n) {
                    showPanelOnResign = YES;
                    [NSApp hide:nil];
                }];

    [NSNotificationCenter.defaultCenter
        addObserverForName:NSApplicationDidResignActiveNotification
                    object:NSApp
                     queue:NSOperationQueue.mainQueue
                usingBlock:^(__unused NSNotification *n) {
                    if (!showPanelOnResign) return;

                    showPanelOnResign = NO;
                    placeOnCursorScreen(w);
                    [w orderFrontRegardless];
                    dispatch_async(dispatch_get_main_queue(), ^{
                        [w makeKeyWindow];
                    });
                }];

    [NSNotificationCenter.defaultCenter
        addObserverForName:NSWindowDidResignKeyNotification
                    object:w
                     queue:NSOperationQueue.mainQueue
                usingBlock:^(__unused NSNotification *n) {
                    [w orderOut:nil];
                }];
}

void zSendCmdV(void) {
    CGEventSourceRef source = CGEventSourceCreate(kCGEventSourceStateHIDSystemState);
    CGEventRef cmdVDown = CGEventCreateKeyboardEvent(source, 0x09, true);
    CGEventRef cmdVUp = CGEventCreateKeyboardEvent(source, 0x09, false);

    CGEventSetFlags(cmdVDown, kCGEventFlagMaskCommand);
    CGEventSetFlags(cmdVUp, kCGEventFlagMaskCommand);

    CGEventPost(kCGHIDEventTap, cmdVDown);
    CGEventPost(kCGHIDEventTap, cmdVUp);

    CFRelease(cmdVDown);
    CFRelease(cmdVUp);
    CFRelease(source);
}

*/
import "C"
import "unsafe"

func SetupPanelNotifications(window unsafe.Pointer) {
	C.zSetupPanelNotifications(window)
}

func ShowPanel(window unsafe.Pointer) {
	C.zShowPanel(window)
}

func Paste() {
	C.zSendCmdV()
}
