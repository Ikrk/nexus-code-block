import AppKit
import Foundation

let html = FileHandle.standardInput.readDataToEndOfFile()
if let htmlString = String(data: html, encoding: .utf8) {
    let pasteboard = NSPasteboard.general
    pasteboard.clearContents()
    pasteboard.declareTypes([.html, .string], owner: nil)
    pasteboard.setString(htmlString, forType: .html)
    pasteboard.setString(htmlString, forType: .string)
}
