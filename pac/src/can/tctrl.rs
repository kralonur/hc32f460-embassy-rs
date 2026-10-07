#[doc = "Register `TCTRL` reader"]
pub type R = crate::R<TctrlSpec>;
#[doc = "Register `TCTRL` writer"]
pub type W = crate::W<TctrlSpec>;
#[doc = "Field `TSSTAT` reader - desc TSSTAT"]
pub type TsstatR = crate::FieldReader;
#[doc = "Field `TTTBM` reader - desc TTTBM"]
pub type TttbmR = crate::BitReader;
#[doc = "Field `TTTBM` writer - desc TTTBM"]
pub type TttbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMODE` reader - desc TSMODE"]
pub type TsmodeR = crate::BitReader;
#[doc = "Field `TSMODE` writer - desc TSMODE"]
pub type TsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSNEXT` reader - desc TSNEXT"]
pub type TsnextR = crate::BitReader;
#[doc = "Field `TSNEXT` writer - desc TSNEXT"]
pub type TsnextW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc TSSTAT"]
    #[inline(always)]
    pub fn tsstat(&self) -> TsstatR {
        TsstatR::new(self.bits & 3)
    }
    #[doc = "Bit 4 - desc TTTBM"]
    #[inline(always)]
    pub fn tttbm(&self) -> TttbmR {
        TttbmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&self) -> TsmodeR {
        TsmodeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&self) -> TsnextR {
        TsnextR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - desc TTTBM"]
    #[inline(always)]
    pub fn tttbm(&mut self) -> TttbmW<'_, TctrlSpec> {
        TttbmW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&mut self) -> TsmodeW<'_, TctrlSpec> {
        TsmodeW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&mut self) -> TsnextW<'_, TctrlSpec> {
        TsnextW::new(self, 6)
    }
}
#[doc = "desc TCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TctrlSpec;
impl crate::RegisterSpec for TctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tctrl::R`](R) reader structure"]
impl crate::Readable for TctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tctrl::W`](W) writer structure"]
impl crate::Writable for TctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCTRL to value 0x90"]
impl crate::Resettable for TctrlSpec {
    const RESET_VALUE: u8 = 0x90;
}
