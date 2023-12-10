import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flip_card/flip_card.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:qr_flutter/qr_flutter.dart';
import 'package:qrscan/qrscan.dart' as scanner;
import 'package:http/http.dart' as http;

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '국방 모바일 공무원증',
      theme: ThemeData(
        primaryColor: Color(0xFF00205B), // Dark navy blue color
        secondaryHeaderColor: Colors.white, // White as an accent color
        scaffoldBackgroundColor: Color(0xFFD8D8D8), // Light grey background
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _currentIndex = 0;
  String my_did = "38C50A5EC714EDA6";
  String? _output = "Empty scan Code";
  GlobalKey<FlipCardState> cardKey = GlobalKey<FlipCardState>();

  Future _scan() async {
    String? did_str = await scanner.scan();
    // Send the did_str to verify api
    final response = await new Session().post(
        'http://192.168.0.50:8000/verify_cert',
        {"cert_did": did_str!, "cert_info": ""});
    debugPrint(response.toString());
    if (response["value"]["value"]) {
      Fluttertoast.showToast(
          msg: "인가된 사용자 입니다!", toastLength: Toast.LENGTH_LONG);
    } else {
      Fluttertoast.showToast(
          msg: "인가되지 않은 사용자 입니다!", toastLength: Toast.LENGTH_LONG);
    }
  }

  Future get_did() async {
    final did = await new Session()
        .post('http://192.168.0.50:8000/get_did', {"user_id": "boyeong"});
    if (did['value']['did'] != null) {
      setState(() {
        my_did = did['value']['did'];
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        toolbarHeight: 80,
        backgroundColor: Color(0xFF00205B),
        title: Text(
          '국방 모바일 공무원증',
          style: TextStyle(
            fontSize: 32,
            fontWeight: FontWeight.w900,
            color: Colors.white,
          ),
        ),
      ),
      body: Center(
        child: FlipCard(
          key: cardKey,
          direction: FlipDirection.HORIZONTAL,
          front: _buildFrontCard(),
          back: _buildBackCard(),
        ),
      ),
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _currentIndex,
        selectedItemColor: Color(0xFF00205B),
        onTap: (index) {
          setState(() {
            _currentIndex = index;
          });
        },
        items: [
          BottomNavigationBarItem(
            icon:
                IconButton(icon: Icon(Icons.home), onPressed: () => get_did()),
            label: 'Home',
          ),
          BottomNavigationBarItem(
            icon:
                IconButton(icon: Icon(Icons.camera), onPressed: () => _scan()),
            label: 'Settings',
          ),
        ],
      ),
    );
  }

  Widget _buildFrontCard() {
    return Container(
      width: 300,
      height: 500,
      decoration: BoxDecoration(
        color: Theme.of(context).primaryColor,
        borderRadius: BorderRadius.circular(20),
        image: DecorationImage(
          image: AssetImage(
              'assets/front_card_background.jpg'), // Add your own image asset
          fit: BoxFit.fill,
        ),
      ),
      child: Column(
        children: [
          SizedBox(height: 130),
          Container(
            width: 180, // Adjust the width as needed
            height: 240, // Adjust the height as needed
            decoration: BoxDecoration(
              color: Colors.white,
              borderRadius: BorderRadius.circular(5),
              image: DecorationImage(
                image:
                    AssetImage('assets/photo.jpg'), // Add your own image asset
                fit: BoxFit.cover,
              ),
            ),
          ),
          SizedBox(height: 1),
          Text(
            '박보영',
            style: TextStyle(
              fontSize: 32,
              fontWeight: FontWeight.w900,
              letterSpacing: 10.0,
              color: Colors.black,
            ),
          ),
          SizedBox(height: 10),
          Text(
            '국방부',
            style: TextStyle(
              fontSize: 37,
              fontWeight: FontWeight.w900,
              letterSpacing: 10.0,
              color: Colors.black,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBackCard() {
    return Container(
      padding: EdgeInsets.only(top: 20),
      width: 300,
      height: 500,
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(20),
        image: DecorationImage(
          image: AssetImage(
              'assets/back_card_background.jpg'), // Add your own image asset
          fit: BoxFit.cover,
        ),
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Container(
            width: 100,
            height: 100,
            decoration: BoxDecoration(
              border: Border.all(
                  color: Theme.of(context).secondaryHeaderColor, width: 3),
              borderRadius: BorderRadius.circular(10),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                QrImageView(
                  data: my_did,
                  version: QrVersions.auto,
                  backgroundColor: Colors.white,
                ),
              ],
            ),
          ),
          SizedBox(height: 20),
          Expanded(
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                // Left Section
                Container(
                  width: 150,
                  child: Text(
                    '               소속\n\n               직위/직급\n\n               성명\n\n               군번\n\n               생년월일',
                    style: TextStyle(
                      color: Colors.black,
                      fontSize: 15,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
                // Center Section (T-Junction)
                Container(
                  width: 150,
                  child: Text(
                    '국 방 부\n\n육 군 대 위\n\n박 보 영\n\n(육)15-33121\n\n1990년 4월 12일',
                    style: TextStyle(
                      color: Colors.black,
                      fontSize: 15,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    _startQRCodeTimer();
  }

  void _startQRCodeTimer() {
    Future.delayed(Duration(seconds: 5), () {
      setState(() {
        cardKey.currentState!.toggleCard();
      });
    });
  }
}

class Session {
  Map<String, String> headers = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  };

  Map<String, String> cookies = {};

  Future<dynamic> post(String url, dynamic data) async {
    http.Response response = await http.post(Uri.parse(Uri.encodeFull(url)),
        body: json.encode(data), headers: headers);
    final int statusCode = response.statusCode;
    if (statusCode < 200 || statusCode > 400 || json == null) {
      return "";
    }
    debugPrint(response.toString());
    return json.decode(utf8.decode(response.bodyBytes));
  }
}
