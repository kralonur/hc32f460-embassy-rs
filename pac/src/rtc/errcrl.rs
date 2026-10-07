#[doc = "Register `ERRCRL` reader"]
pub type R = crate::R<ErrcrlSpec>;
#[doc = "Register `ERRCRL` writer"]
pub type W = crate::W<ErrcrlSpec>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type CompR = crate::FieldReader;
#[doc = "Field `COMP` writer - desc COMP"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, ErrcrlSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "desc ERRCRL\n\nYou can [`read`](crate::Reg::read) this register and get [`errcrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcrlSpec;
impl crate::RegisterSpec for ErrcrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`errcrl::R`](R) reader structure"]
impl crate::Readable for ErrcrlSpec {}
#[doc = "`write(|w| ..)` method takes [`errcrl::W`](W) writer structure"]
impl crate::Writable for ErrcrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRCRL to value 0x20"]
impl crate::Resettable for ErrcrlSpec {
    const RESET_VALUE: u8 = 0x20;
}
