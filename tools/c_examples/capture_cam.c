int CaptureCam() {

  avdevice_register_all(); // for device 
  avcodec_register_all();
  av_register_all();

  char *dev_name = "/dev/video0"; // here mine is video0 , it may vary.
  AVInputFormat *inputFormat =av_find_input_format("v4l2");
  AVDictionary *options = NULL;
  av_dict_set(&options, "framerate", "20", 0);

  AVFormatContext *pAVFormatContext = NULL; 

    // check video source
  if(avformat_open_input(&pAVFormatContext, dev_name, inputFormat, NULL) != 0)
  {
   cout<<"\nOops, could'nt open video source\n\n";
   return -1;
  }
  else
  {
   cout<<"\n Success !";
  }

  } // end function
