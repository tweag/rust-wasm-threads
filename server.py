from http.server import HTTPServer, SimpleHTTPRequestHandler, test
import sys

class ReqHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        SimpleHTTPRequestHandler.end_headers(self)

if __name__ == '__main__':
    test(ReqHandler, HTTPServer, port=8001)
